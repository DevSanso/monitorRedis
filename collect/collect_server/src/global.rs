use std::fs;

use once_cell::sync::OnceCell;

use core::utils_new_error;
use crate::args::Args;
use crate::config::Config;

use dbs::redis_pool::{RedisPoolAlias, new_redis_pool};

use dbs::sqlite_pool::{SqlitePoolAlias, new_sqlite_pool};
use dbs::pg_pool::{PgPoolAlias, new_pg_pool};
use dbs::utils::{create_pg_url, create_redis_url};

pub struct GlobalPool<T :'static> {
    pub server_pool : T,
    pub manage_pool : SqlitePoolAlias,
    pub collect_pool : PgPoolAlias
}
pub struct Global<T : 'static> {
    pub args : Args,
    pub pools : GlobalPool<T> 
}

fn get_pool(cfg : &Config) -> (PgPoolAlias, SqlitePoolAlias) {
    let pg_url = create_pg_url(cfg.pg_config.user.as_str(), 
    cfg.pg_config.password.as_str(), 
    cfg.pg_config.ip.as_str(), 
    cfg.pg_config.port, 
    cfg.pg_config.db_name.as_str());

    let pg_p = new_pg_pool(String::from("collectDB"), pg_url, 30);
    let sqlite_p = new_sqlite_pool(String::from("manageDB"), cfg.sqlite_path.to_string(), 2);
    
    (pg_p, sqlite_p)
}

static mut REDIS_GLOBAL : OnceCell<Global<RedisPoolAlias>> = OnceCell::new();

pub fn get_redis_global() -> &'static Global<RedisPoolAlias> {
    unsafe {
        REDIS_GLOBAL.get_unchecked()
    }
}

pub fn init_global(arg : Args) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::new_from_json(
        fs::read_to_string(arg.conf_path.clone())?.as_str()
    )?;

    let ps = get_pool(&cfg);

    if arg.server_type.as_str() == "redis" {
        let redis_conn_info = {
            let manage_conn = ps.1.get_owned(())?;
            let info = crate::db_info::get_redis_access_datas(manage_conn, arg.server_id)?;

            info
        };

        if redis_conn_info.len() <= 0 {
            return utils_new_error!(fetch, GetFailedError, format!("not exist redis connection this server_id={}", arg.server_id));
        }
        let info = &redis_conn_info[0].conn_cfg;

        let redis_p = new_redis_pool("redisDB".to_string(), 
        create_redis_url(info.user.as_str(), info.password.as_str(), info.ip.as_str(), info.port, info.db_name), 10);
        
        unsafe {
            REDIS_GLOBAL.set(Global {
                args : arg,
                pools : GlobalPool {
                    server_pool : redis_p,
                    collect_pool : ps.0,
                    manage_pool : ps.1
                }
            });
        }
    }
    else {
        return utils_new_error!(proc, NoneDataError, format!("not support this server_type {}", arg.server_type));
    }


    Ok(())
}



