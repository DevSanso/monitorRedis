use core::utils_inherit_error;
use std::env;
use std::fs;
use std::error::Error;
use std::io::Read;
use std::thread;
use std::time::Duration;

use serde_json;
use md5::{Md5, Digest};

use core::structure::pool::PoolItem;
use dbs::sqlite_pool::{SqliteRows,SqliteConn};
use dbs_cmd::{SQLITE_COMMANDLINE_MAP, SQLiteCommand};

use crate::typed::{RedisConnCfg, DbConnConfig};

fn get_fetch_redis_access_data(rows : SqliteRows) -> Result<Vec<RedisConnCfg>, Box<dyn Error>> {
    
    let mut v = Vec::new();
    for row in 0..rows.row_len() {

        let id = rows.get_i64_data(row, 0)?.unwrap() as i32;
        let user = rows.get_str_data(row, 1)?.unwrap();
        let password = rows.get_str_data(row, 2)?.unwrap();
        let  db_name = rows.get_i64_data(row, 3)?.unwrap() as u32;
        let ip = rows.get_str_data(row, 4)?.unwrap();
        let port =  rows.get_i64_data(row, 5)?.unwrap() as u32;

        let mut hasher = Md5::new();
        hasher.update(ip.as_bytes());
        let result = hasher.finalize();

        let conn = RedisConnCfg {
            link_id : id,
            conn_cfg : DbConnConfig {
                user : user,
                password : password,
                db_name : db_name,
                ip : ip,
                port : port
            },
            hash_code : Vec::from(result.as_slice())
        };

        v.push(conn);
    };

    Ok(v)
}


pub(crate) fn get_redis_access_datas(mut sqlite_item : PoolItem<'_, SqliteConn>) ->Result<Vec<RedisConnCfg>, Box<dyn Error>> {
    let sql_conn = sqlite_item.get_value();

    match sql_conn.query(SQLITE_COMMANDLINE_MAP.get(&SQLiteCommand::RedisConnInfo).unwrap().to_string(), &[], get_fetch_redis_access_data, "get_redis_access_datas") {
        Ok(ok) => Ok(ok),
        Err(err) => utils_inherit_error!(fetch, GetFailedError, "get_redis_access_datas", err)
    }
}