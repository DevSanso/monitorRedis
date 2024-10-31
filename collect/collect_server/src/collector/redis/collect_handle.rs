use std::error::Error;
use std::collections::HashMap;

use crate::global::get_redis_global;
use core::utils_new_error;
use dbs::pg_pool::PgUploader;
use dbs_cmd::{COLLECT_COMMANDLINE_MAP, CollectCommand};


use crate::utils::parsing::redis_res::*;
use crate::utils::parsing::redis_res::KeyMemUsage;


pub fn client_list_handle(server_id : i32, val : String) -> Result<(), Box<dyn Error>> {
    let list = parsing_client_list(val)?;
    let mut client_iter = list.iter();

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();


    let mut t = collect_conn.trans()?;
    let pg_query = dbs_cmd::COLLECT_COMMANDLINE_MAP.get(&CollectCommand::RedisClientList).unwrap();
    
    loop {
        let seq = client_iter.next();
        if seq.is_none() {
            let _ = t.commit();
            break;
        }

        let client = seq.unwrap();
        let exec_ret =  t.execute(&pg_query, &[&server_id,
            &client.id, &client.addr, &client.fd, &client.name, &client.age,
            &client.idle, &String::from(client.flags), &client.db, &client.sub,
            &client.psub, &(client.multi as i64), &client.qbuf, &client.qbuf_free,
            &client.obl, &client.oll, &client.omem, &String::from(client.events), &client.cmd, &client.user]);

        if exec_ret.is_err() {
            let _ = t.rollback();
            return Err(exec_ret.unwrap_err());
        }
    }

    Ok(())
}