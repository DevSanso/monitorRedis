use core::utils_inherit_error;
use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;

use crate::utils::parsing::redis_res::parsing_info_keyspace;

pub fn info_keyspace_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    redis_conn.set_app_name("collect_info_keyspace")?;
    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InsertInfoKeySpace).unwrap();
    let keyspace_cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::InfoKeySpace).unwrap();

    let db_size_res = redis_conn.run_command(keyspace_cmd, &[])?;    
    let infos = parsing_info_keyspace(db_size_res)?;

    let mut trans = pg_conn.trans()?;

    for item in infos {
        match trans.execute(&pg_query, &[&link_key, &item.db_name, &item.key_cnt, &item.expire_cnt, &item.avg_ttl]) {
            Err(err) => {
                let _ = trans.rollback();
                return utils_inherit_error!(connection, CommandRunError, "", err)
            }
            Ok(_) => {}
        }
    }

    let _ = trans.commit();

    Ok(())
}