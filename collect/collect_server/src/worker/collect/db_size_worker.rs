use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;

use parsing::redis_res::{parsing_confg_get_databases, parsing_dbsize};

pub fn db_size_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetDatabaseCount).unwrap();
 
    let result = redis_conn.run_command(cmd, &[])?;
    let dbnums = parsing_confg_get_databases(result)?;

    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::DbSize).unwrap();
    let db_size_cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::DbSize).unwrap();

    for db in 0..dbnums {
        let db_size_res = redis_conn.run_command(db_size_cmd, &[db.to_string().as_str()])?;    
        let dbsize = parsing_dbsize(db_size_res)?;

        pg_conn.execute(&pg_query, &[&link_key, &db, &dbsize])?;
    }

    Ok(())
}