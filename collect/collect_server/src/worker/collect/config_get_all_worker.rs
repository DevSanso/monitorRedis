use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;

use crate::utils::parsing::redis_res::parsing_config_get_all;

pub fn config_get_all_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    redis_conn.set_app_name("collect_config_get")?;
    let redis_cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetAllConfig).unwrap();

    let ret = redis_conn.run_command(&redis_cmd, &[])?;
    let parsing_data = parsing_config_get_all(ret)?;
    let mut config_iter = parsing_data.iter();


    let mut t = pg_conn.trans()?;
    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::ConfigAll).unwrap();
    
    loop {
        let seq = config_iter.next();
        if seq.is_none() {
            let _ = t.commit();
            break;
        }

        let conf = seq.unwrap();
        let exec_ret = t.execute(&pg_query, &[&link_key, &conf.name, &conf.value]);

        if exec_ret.is_err() {
            let _ = t.rollback();
            return Err(exec_ret.unwrap_err());
        }
    }

    Ok(())
}