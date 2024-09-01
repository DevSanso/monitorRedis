use std::error::Error;

use dbs::{self, pg_pool::PgUploader};
use dbs_cmd;

use crate::utils::parsing::redis_res::parsing_info_memory;
use dbs::utils::make_pg_numeric;

pub fn info_memory_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    redis_conn.set_app_name("collect_info_stat")?;
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::InfoMemory).unwrap();
    let result = redis_conn.run_command(cmd, &[])?;

    let mem = parsing_info_memory(result)?;

    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InsertInfoMemory).unwrap();
    pg_conn.execute(&pg_query, &[
        &link_key, 
        &mem.used_memory, 
        &mem.used_memory_rss, 
        &mem.used_memory_peak, 
        &mem.used_memory_overhead, 
        &mem.used_memory_dataset,
        &mem.allocator_allocated,
        &mem.used_memory_lua,
        &mem.used_memory_scripts,
        &mem.maxmemory,
        &mem.maxmemory_policy.as_str(),
        &mem.mem_clients_slaves,
        &mem.mem_clients_normal,
        &mem.mem_aof_buffer,
        &mem.mem_allocator.as_str()])?;
    Ok(())
}