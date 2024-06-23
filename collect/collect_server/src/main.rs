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

use serde_json;

use dbs::redis_pool::RedisPool;
use dbs::sqlite_pool::SqlitePool;
use dbs::utils::create_redis_url;
use crate::threads::builder::RedisExectorBulider;

use config::Config;
use logger::{init_logger, LoggerConfig};
use dbs::pg_pool::PgPool;
use dbs::utils::create_pg_url;
use worker::collect::make_one_collect_worker;
use core::utils_new_error;

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
        .register_pg(pools.0)
        .register_sqlite(pools.1)
        .set_alloc_size(30)
        .set_name("RedisExector")
        .set_redis_select_fn(&db_info::get_redis_access_datas)
        .register_workers(make_one_collect_worker());

    let mut executor = build.build();

    loop {
        executor.load_redis_connect_info();
        executor.update_run_worker();
        executor.run_worker();
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

#[cfg(feature = "runTest")]
pub fn server_main_test(cfg : Config) -> Result<(), Box<dyn Error>> {
    let log_cfgs = vec![];
    init_logger(log_cfgs)?;
    let mut pools = get_pool(&cfg);

    let redis_list = get_redis_access_datas(&mut pools.1)?;

    let mut build = RedisExectorBulider::new()
        .set_name("executorCtl")
        .set_alloc_size(30)
        .register_pg(pools.0);

    for r in redis_list {
        let cfg = r.1;
        build = build.register_redis(r.0, RedisPool::new(cfg.ip.clone(), 
            create_redis_url(cfg.user.as_str(), cfg.password.as_str(), cfg.ip.as_str(), cfg.port, cfg.db_name)));
    }

    for r in make_one_collect_worker() {
        let f_cfg = r.1;
        build = build.register_worker(r.0, f_cfg.0, f_cfg.1);
    }

    let mut execute = build.build()?;

    loop {
        execute.run_workers();
        thread::sleep(Duration::from_millis(100));
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
