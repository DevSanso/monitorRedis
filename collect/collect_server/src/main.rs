mod config;
mod worker;
mod threads;
mod typed;
mod db_info;


use std::env;
use std::fs;
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use serde_json;
use chrono::Timelike;

use dbs::sqlite_pool::{SqlitePoolAlias, new_sqlite_pool};
use dbs::pg_pool::{PgPoolAlias, new_pg_pool};
use dbs::redis_pool::{RedisPoolAlias, new_redis_pool};
use crate::threads::builder::RedisExectorBulider;

use config::Config;
use logger::{init_logger, LoggerConfig};
use dbs::utils::create_pg_url;
use worker::collect::make_one_collect_worker;
use core::utils_new_error;

fn get_pool(cfg : &Config) -> (PgPoolAlias, SqlitePoolAlias) {
    let pg_url = create_pg_url(cfg.pg_config.user.as_str(), 
    cfg.pg_config.password.as_str(), 
    cfg.pg_config.ip.as_str(), 
    cfg.pg_config.port, 
    cfg.pg_config.db_name.as_str());

    let pg_p = new_pg_pool(String::from("collectDB"), pg_url, 30);
    let sqlite_p = new_sqlite_pool(String::from("infoDB"), cfg.sqlite_path.to_string(), 2);

    (pg_p, sqlite_p)
}

fn get_process_arg() -> Result<String, Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return utils_new_error!(conf, ProcessConfigPathError, args.len());
    }
    Ok(args[0].clone())
}

pub fn server_main(cfg : Config) -> Result<(), Box<dyn Error>> {    
    let log_cfgs = vec![LoggerConfig::new(cfg.logger_level.clone(), cfg.logger_path.clone())];
    init_logger(log_cfgs)?;

    let pools = get_pool(&cfg);

    let build = RedisExectorBulider::new()
        .register_pg(&pools.0)
        .register_sqlite(&pools.1)
        .set_alloc_size(30)
        .set_name("RedisExector")
        .set_redis_select_fn(&db_info::get_redis_access_datas)
        .register_workers(make_one_collect_worker());

    let mut executor = build.build();

    loop {
        let load_ret = executor.load_redis_connect_info(); 
        if load_ret.is_err() {
            log::error!("Main[Err] : {}", load_ret.err().unwrap());
            continue;
        }
        executor.update_run_worker();

        let run_ret = executor.run_worker();
        if run_ret.is_err() {
            log::error!("Main[Err] : {}", run_ret.err().unwrap());
            continue;
        }
        let millis = chrono::Local::now().timestamp_millis() % 1000;
        let delay = 1000 - millis + 5;
        thread::sleep(Duration::from_millis(delay as u64));
    }

    Ok(())
}

#[cfg(feature = "runTest")]
pub fn server_main_test(cfg : Config) -> Result<(), Box<dyn Error>> {
    let log_cfgs = vec![];
    init_logger(log_cfgs)?;
    let mut pools = get_pool(&cfg);

    let pools = get_pool(&cfg);

    let build = RedisExectorBulider::new()
        .register_pg(&pools.0)
        .register_sqlite(&pools.1)
        .set_alloc_size(30)
        .set_name("RedisExector_Test")
        .set_redis_select_fn(&db_info::get_redis_access_datas)
        .register_workers(make_one_collect_worker());

    let mut executor = build.build();

    loop {
        let load_ret = executor.load_redis_connect_info(); 
        if load_ret.is_err() {
            log::error!("Main[Err] : {}", load_ret.err().unwrap());
            continue;
        }
        executor.update_run_worker();
        
        let run_ret = executor.run_worker();
        if run_ret.is_err() {
            log::error!("Main[Err] : {}", run_ret.err().unwrap());
            continue;
        }

        let millis = chrono::Local::now().timestamp_millis() % 1000;
        let delay = 1000 - millis + 5;
        thread::sleep(Duration::from_millis(delay as u64));
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
