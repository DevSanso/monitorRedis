use core::utils_inherit_error;
use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;
use utils::parsing::redis_res::parsing_key_usage_top_one_hundred;

pub fn key_usage_top_one_hundred_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetMemoryKeyUsage500Range).unwrap();
    let result = redis_conn.run_command(cmd, &[&"500"])?;

    let key_usage = parsing_key_usage_top_one_hundred(result)?;


    Ok(())
}