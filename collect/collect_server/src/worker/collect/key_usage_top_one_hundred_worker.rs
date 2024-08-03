use core::utils_inherit_error;
use std::error::Error;
use std::mem::size_of;
use std::time;
use std::thread;
use std::ptr;
use std::array;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;
use md5::digest::Key;
use utils::parsing::redis_res::{parsing_key_usage_top_ten_hundred, KeyMemUsage};

#[inline]
fn push_and_sort_buffer_data(dst : &mut [KeyMemUsage;4500], src : &'_ [KeyMemUsage]) {
    dst[1000..src.len() + 1000].clone_from_slice(src);
    dst.sort_by(|a,b| {a.mem_size.partial_cmp(&b.mem_size).unwrap()});
    dst.reverse();
}

pub fn key_usage_top_ten_hundred_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetMemoryKeyUsage3000Range).unwrap();
    
    let mut cursor = -1;
    let mut buffer : [KeyMemUsage;4500] = array::from_fn(|_| KeyMemUsage::default());

    while cursor != 0 {
        let result_opt = if cursor == -1 {
            Some(redis_conn.run_command(cmd, &[&"0"])?)
        }else {
            let cur = cursor.to_string();
            Some(redis_conn.run_command(cmd, &[&cur.as_str()])?)
        };

        let result = result_opt.unwrap();

        let buf_keyusage = parsing_key_usage_top_ten_hundred(result)?;

        cursor = buf_keyusage.next_cursor;
        push_and_sort_buffer_data(&mut buffer, buf_keyusage.fetch.as_slice());
        thread::sleep(time::Duration::from_millis(1000));
    }

    let mut t = pg_conn.trans()?;
    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InsertKeyUsageTopTenHundred).unwrap();

    for item in buffer.iter().take(1000) {
        let ret = t.execute(pg_query, &[&link_key, &item.name, &item.mem_size, &item.remain_expired_time]);

        if ret.is_err() {
            let _ = t.rollback();
            return Err(ret.err().unwrap());
        }
    }
    let _ = t.commit();

    Ok(())
}