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
use utils::parsing::redis_res::{parsing_key_usage_top_one_hundred, KeyMemUsage};

#[inline]
fn push_and_sort_buffer_data(dst : &mut [KeyMemUsage;602], src : &'_ [KeyMemUsage]) {
    dst[100..src.len() + 100].clone_from_slice(src);
    dst.sort_by(|a,b| {a.mem_size.partial_cmp(&b.mem_size).unwrap()});
    dst.reverse();
}

pub fn key_usage_top_one_hundred_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetMemoryKeyUsage500Range).unwrap();
    
    let mut cursor = -1;
    let mut buffer : [KeyMemUsage;602] = array::from_fn(|_| KeyMemUsage::default());

    while cursor != 0 {
        let result_opt = if cursor == -1 {
            Some(redis_conn.run_command(cmd, &[&"0"])?)
        }else {
            let cur = cursor.to_string();
            Some(redis_conn.run_command(cmd, &[&cur.as_str()])?)
        };

        let result = result_opt.unwrap();

        let buf_keyusage = parsing_key_usage_top_one_hundred(result)?;
        cursor = buf_keyusage.next_cursor;
        
        push_and_sort_buffer_data(&mut buffer, buf_keyusage.fetch.as_slice());
        thread::sleep(time::Duration::from_millis(5));
    }
    

    Ok(())
}