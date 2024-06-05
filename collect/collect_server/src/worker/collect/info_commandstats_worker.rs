use std::error::Error;

use dbs;
use dbs_cmd;

use parsing::redis_res::parsing_info_commandstats;

pub fn info_commandstats_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgUploader) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::InfoCommandStats).unwrap();
    let result = redis_conn.run_command(cmd, &[])?;
    
    let list = parsing_info_commandstats(result)?;
    let mut stat_iter = list.iter();

    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InfoCommandStats).unwrap();

    let mut t = pg_conn.trans()?;
    
    loop {
        let seq = stat_iter.next();
        if seq.is_none() {
            let _ = t.commit();
            break;
        }

        let stats = seq.unwrap();
        let exec_ret = t.execute(&pg_query, &[&link_key, &stats.cmd, &stats.calls, &stats.usec, &stats.usec_per_call]);

        if exec_ret.is_err() {
            let _ = t.rollback();
            return Err(exec_ret.unwrap_err());
        }
    }

    Ok(())
}