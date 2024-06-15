use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;

use utils::parsing::redis_res::parsing_info_cpu;
use dbs::utils::make_pg_numeric;

pub fn info_cpu_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::InfoCpu).unwrap();
    let result = redis_conn.run_command(cmd, &[])?;

    let c = parsing_info_cpu(result)?;


    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InfoCpu).unwrap();
    pg_conn.execute(&pg_query, &[&link_key, &make_pg_numeric(c.cpu_sys), &make_pg_numeric(c.cpu_user), &make_pg_numeric(c.child_cpu_sys), &make_pg_numeric(c.child_cpu_user)])?;
    Ok(())
}