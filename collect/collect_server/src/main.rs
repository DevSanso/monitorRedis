mod config;
mod worker;
mod errors;
mod threads;

use std::env;
use std::fs;
use std::error::Error;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use dbs::redis_pool::RedisPool;
use dbs::sqlite_pool::SqlitePool;
use dbs::utils::create_redis_url;
use serde_json;

use config::Config;
use logger::{init_logger, LoggerConfig};
use dbs::pg_pool::PgPool;
use dbs::utils::create_pg_url;
use threads::ThreadExecuter;
use worker::collect::make_sec_worker;

fn get_pool(cfg : &Config) -> (Arc<Mutex<PgPool>>, SqlitePool) {
    let pg_url = create_pg_url(cfg.pg_config.user.as_str(), 
    cfg.pg_config.password.as_str(), 
    cfg.pg_config.ip.as_str(), 
    cfg.pg_config.port, 
    cfg.pg_config.db_name.as_str());

    let pg_p = PgPool::new(pg_url);
    let sqlite_p = SqlitePool::new(cfg.sqlite_path.clone());

    (Arc::new(Mutex::new(pg_p)), sqlite_p)
}

fn get_redis_access_datas(sqlite_p : &mut SqlitePool) ->Result<Vec<(u32,config::DbConnConfig<u32>)>, Box<dyn Error>> {
    let mut sql_item = sqlite_p.get()?;
    let sql_conn = sql_item.get_value();

    let mut stmt = sql_conn.prepare(dbs_cmd::SQLITE_COMMANDLINE_MAP.get(&dbs_cmd::SQLiteCommand::RedisConnInfo).unwrap())?;
    let ret = stmt.query_map([], |row| {
        let unqiue_id : u32 = row.get(0)?;
        Ok((unqiue_id, config::DbConnConfig::<u32> {
            user : row.get(1)?,
            password : row.get(2)?,
            db_name : row.get(3)?,
            ip : row.get(4)?,
            port : row.get(5)?
        }))
    })?;

    let mut v = Vec::new();
    for r in ret {
        let row = r.unwrap();
        v.push(row);
    }
    
    Ok(v)
}

fn make_redis_monitor(v : Vec<(u32,config::DbConnConfig<u32>)>, global_pg_p : &'_ Arc<Mutex<PgPool>>) -> Vec<ThreadExecuter> {
    let ret : Vec<ThreadExecuter> = v.iter().fold(vec![], |mut acc, x| {
        let conn_info = &x.1;
        let clone_pg_p = Arc::clone(global_pg_p);

        let redis_p = RedisPool::new(create_redis_url(conn_info.user.as_str(), 
        conn_info.password.as_str(), conn_info.ip.as_str(), conn_info.port, conn_info.db_name));

        let execute = ThreadExecuter::new(x.0, redis_p, clone_pg_p, make_sec_worker());
        acc.push(execute);
        acc
    });

    ret
}

fn process_mon_loop() {
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(errors::MoreArgsError));
    }

    let config_str = fs::read_to_string(args[1].clone())?;

    let cfg : Config = serde_json::from_str(config_str.as_str())?;
    
    let log_cfgs = vec![LoggerConfig::new(cfg.logger_level.clone(), cfg.logger_path.clone())];
    init_logger(log_cfgs)?;

    let mut pools = get_pool(&cfg);

    let redis_list = get_redis_access_datas(&mut pools.1)?;

    let mut execs = make_redis_monitor(redis_list, &pools.0);

    for exec in &mut execs {
        exec.auto_no_block_run();
    }
    
    process_mon_loop();

    Ok(())
}
