mod collect_handle;

use std::error::Error;
use std::collections::HashMap;

use core::utils_new_error;
use dbs_cmd::{RedisCommand, REIDS_COMMANDLINE_MAP, CollectCommand};
use dbs::redis_pool::{RedisPoolAlias, RedisRequester};

use crate::utils::parsing::redis_res::KeyMemUsage;
pub struct RedisCollector {
    pool : RedisPoolAlias,
    command : RedisCommand
}

pub static MAPPING_REDIS_AND_COLLECT : once_cell::sync::Lazy<HashMap<RedisCommand, Vec<CollectCommand>>> =
    once_cell::sync::Lazy::new(|| {
        let mut map_internal = HashMap::new();

        map_internal.insert(RedisCommand::ClientList, vec![CollectCommand::RedisClientList]);
        map_internal
    });

fn no_param_cmd(conn : &'_ mut RedisRequester, command : &'_ RedisCommand) -> Result<(), Box<dyn Error>>{
    let cmd = REIDS_COMMANDLINE_MAP.get(command).unwrap();
    
    let res = conn.run_command(cmd, &[])?;
    let collect_querys = MAPPING_REDIS_AND_COLLECT.get(command).ok_or::<Box<dyn Error>>(utils_new_error!(proc, CriticalError,""))?;



    Ok(())
}

fn key_usage_top_one_hundred_cmd(conn : &'_ mut RedisRequester, command : &'_ RedisCommand) -> Result<(), Box<dyn Error>> {

    Ok(())
}

impl crate::collector::Collector<dbs::redis_pool::RedisRequester, String> for RedisCollector {
    fn run_collect(&mut self) -> Result<(), Box<dyn Error>> {
        let mut item = {
            self.pool.get_owned(())?
        };

        let conn = item.get_value();

        conn.set_app_name("collect")?;

        let cmd = REIDS_COMMANDLINE_MAP.get(&self.command).unwrap();

        if self.command == RedisCommand::GetMemoryKeyUsage3000Range {
            key_usage_top_one_hundred_cmd(conn, &self.command)
        }
        else {
            conn.run_command(cmd, &[]);
            Ok(())
        }
    }
}

pub fn make_redis_collector(pool : RedisPoolAlias, command : RedisCommand) -> Box<dyn crate::collector::Collector<RedisRequester, String>> {
    let c = RedisCollector{pool, command};
    Box::new(c)
}