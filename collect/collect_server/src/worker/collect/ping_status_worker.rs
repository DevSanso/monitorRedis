use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;

use log::info;

pub fn ping_status_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let result = match redis_conn.ping() {
        Ok(_) => true,
        Err(err) =>  {
            info!("ping_status_worker[{}] - ping command failed [{}]", link_key, err.to_string());
            false
        }
    };

    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::PingUpdate).unwrap();


    pg_conn.execute(&pg_query,
         &[&link_key, if result {&"Y"} else {&"N"}])?;

    
    Ok(())
}