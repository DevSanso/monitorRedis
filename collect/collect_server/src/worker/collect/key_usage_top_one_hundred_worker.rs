use core::utils_inherit_error;
use std::error::Error;
use std::time;
use std::thread;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;
use utils::parsing::redis_res::{parsing_key_usage_top_one_hundred, KeyMemUsage};

#[inline]
fn push_and_sort_buffer_data(dst : &mut Vec<KeyMemUsage>, src : &'_ [KeyMemUsage]) {
    dst.clone_from_slice(src);
    dst.sort_by(|a,b| a.mem_size.partial_cmp(&b.mem_size));
    let _ = dst.drain(100..);
}

pub fn key_usage_top_one_hundred_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetMemoryKeyUsage500Range).unwrap();
    
    let mut cursor = -1;
    let mut buffer = Vec::<KeyMemUsage>::with_capacity(601);

    while cursor != 0 {
        let args = if cursor == -1 {
            &["0"]
        }else {
            &[String::from(cursor).as_str()]
        };

        let result = redis_conn.run_command(cmd, &args)?;

        let buf_keyusage = parsing_key_usage_top_one_hundred(result)?;
        cursor = buf_keyusage.next_cursor;
        
        push_and_sort_buffer_data(&mut buffer, buf_keyusage.fetch.as_slice());
        thread::sleep(time::Duration::from_millis(5));
    }

    
    

    Ok(())
}