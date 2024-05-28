mod config;
mod worker;
mod errors;
mod threads;
mod typed;

use std::env;
use std::fs;
use std::error::Error;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use serde_json;

use dbs::redis_pool::RedisPool;
use dbs::sqlite_pool::SqlitePool;
use dbs::utils::create_redis_url;
use crate::threads::builder::ExectorBulider;

use config::Config;
use logger::{init_logger, LoggerConfig};
use dbs::pg_pool::PgPool;
use dbs::utils::create_pg_url;
use worker::collect::make_sec_worker;

fn get_pool(cfg : &Config) -> (PgPool, SqlitePool) {
    let pg_url = create_pg_url(cfg.pg_config.user.as_str(), 
    cfg.pg_config.password.as_str(), 
    cfg.pg_config.ip.as_str(), 
    cfg.pg_config.port, 
    cfg.pg_config.db_name.as_str());

    let pg_p = PgPool::new(pg_url);
    let sqlite_p = SqlitePool::new(cfg.sqlite_path.clone());

    (pg_p, sqlite_p)
}

fn get_redis_access_datas(sqlite_p : &mut SqlitePool) ->Result<Vec<(i32,config::DbConnConfig<u32>)>, Box<dyn Error>> {
    let mut sql_item = sqlite_p.get()?;
    let sql_conn = sql_item.get_value();

    let mut stmt = sql_conn.prepare(dbs_cmd::SQLITE_COMMANDLINE_MAP.get(&dbs_cmd::SQLiteCommand::RedisConnInfo).unwrap())?;
    let ret = stmt.query_map([], |row| {
        let unqiue_id : i32 = row.get(0)?;
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

fn get_process_arg() -> Result<String, Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(errors::MoreArgsError));
    }
    Ok(args[0].clone())
}

pub fn server_main(cfg : Config) -> Result<(), Box<dyn Error>> {    
    let log_cfgs = vec![LoggerConfig::new(cfg.logger_level.clone(), cfg.logger_path.clone())];
    init_logger(log_cfgs)?;

    let mut pools = get_pool(&cfg);

    let redis_list = get_redis_access_datas(&mut pools.1)?;

    let mut build = ExectorBulider::new()
        .set_name("executorCtl")
        .set_alloc_size(30)
        .register_pg(pools.0);

    for r in redis_list {
        let cfg = r.1;
        build = build.register_redis(r.0, RedisPool::new(cfg.ip.clone(), 
            create_redis_url(cfg.user.as_str(), cfg.password.as_str(), cfg.ip.as_str(), cfg.port, cfg.db_name)));
    }

    for r in make_sec_worker() {
        let f_cfg = r.1;
        build = build.register_worker(r.0, f_cfg.0, f_cfg.1);
    }

    let mut execute = build.build()?;

    loop {
        execute.run_workers();
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

#[cfg(feature = "runTest")]
pub fn server_main_test(cfg : Config) -> Result<(), Box<dyn Error>> {
    let log_cfgs = vec![];
    init_logger(log_cfgs)?;
    let mut pools = get_pool(&cfg);

    let redis_list = get_redis_access_datas(&mut pools.1)?;

    let mut build = ExectorBulider::new()
        .set_name("executorCtl")
        .set_alloc_size(30)
        .register_pg(pools.0);

    for r in redis_list {
        let cfg = r.1;
        build = build.register_redis(r.0, RedisPool::new(cfg.ip.clone(), 
            create_redis_url(cfg.user.as_str(), cfg.password.as_str(), cfg.ip.as_str(), cfg.port, cfg.db_name)));
    }

    for r in make_sec_worker() {
        let f_cfg = r.1;
        build = build.register_worker(r.0, f_cfg.0, f_cfg.1);
    }

    let mut execute = build.build()?;

    loop {
        execute.run_workers();
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    #[cfg(not(feature = "runTest"))]
    let arg = get_process_arg()?;

    #[cfg(not(feature = "runTest"))]
    let config_str = fs::read_to_string(arg.clone())?;

    #[cfg(feature = "runTest")]
    let config_str = String::from(include_str!(
        "../../assets/test/config/server_test.json"
    ));

    let cfg : Config = serde_json::from_str(config_str.as_str())?;


    #[cfg(not(feature = "runTest"))]
    server_main(cfg)?;
    #[cfg(feature = "runTest")]
    server_main_test(cfg)?;

    Ok(())
}
