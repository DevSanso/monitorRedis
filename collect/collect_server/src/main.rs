mod config;
mod worker;
mod errors;
mod thread_pool;

use std::env;
use std::fs;
use std::error::Error;

use serde_json;

use config::Config;
use logger::{init_logger, LoggerConfig};
use dbs::pg_pool::PgPool;
use dbs::utils::create_pg_url;

fn main() -> Result<(), Box<dyn Error>>{
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(errors::MoreArgsError));
    }

    let config_str = fs::read_to_string(args[1].clone())?;

    let cfg : Config = serde_json::from_str(config_str.as_str())?;
    
    let log_cfgs = vec![LoggerConfig::new(cfg.logger_level, cfg.logger_path)];
    init_logger(log_cfgs)?;

    let pg_url = create_pg_url(cfg.pg_config.user.as_str(), 
        cfg.pg_config.password.as_str(), 
        cfg.pg_config.ip.as_str(), 
        cfg.pg_config.port, 
        cfg.pg_config.db_name.as_str());

    let pg_p = PgPool::new(pg_url);

    Ok(())
}
