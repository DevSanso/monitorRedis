mod collect_handle;

use std::error::Error;
use std::collections::HashMap;

use collect_handle::*;

use crate::global::get_redis_global;
use core::utils_new_error;
use dbs_cmd::{RedisCommand, REIDS_COMMANDLINE_MAP, CollectCommand};
use dbs::redis_pool::{RedisPoolAlias, RedisRequester};

use crate::utils::parsing::redis_res::KeyMemUsage;
pub struct RedisCollector {
    server_id : i32,
    command : RedisCommand
}

fn no_param_cmd(server_id : i32, conn : &'_ mut RedisRequester, command : &'_ RedisCommand) -> Result<(), Box<dyn Error>>{
    let cmd = REIDS_COMMANDLINE_MAP.get(command).unwrap();
    let res = conn.run_command(cmd, &[])?;
    
    match command {
        RedisCommand::ClientList => client_list_handle(server_id, res),
        _ => utils_new_error!(proc, UnkownError, "unkown command handle")
    }?;


    Ok(())
}

fn key_usage_top_one_hundred_cmd(conn : &'_ mut RedisRequester, command : &'_ RedisCommand) -> Result<(), Box<dyn Error>> {

    Ok(())
}

impl crate::collector::Collector<dbs::redis_pool::RedisRequester, String> for RedisCollector {
    fn run_collect(&mut self) -> Result<(), Box<dyn Error>> {
        let mut item = {
            get_redis_global().pools.server_pool.get_owned(())?
        };

        let conn = item.get_value();

        conn.set_app_name("collect redis")?;

        let cmd = REIDS_COMMANDLINE_MAP.get(&self.command).unwrap();

        if self.command == RedisCommand::GetMemoryKeyUsage3000Range {
            key_usage_top_one_hundred_cmd(conn, &self.command)
        }
        else {
            no_param_cmd(self.server_id, conn, &self.command);
            Ok(())
        }
    }
}

pub fn make_redis_collector(server_id : i32, command : RedisCommand) -> Box<dyn crate::collector::Collector<RedisRequester, String>> {
    let c = RedisCollector{server_id, command};
    Box::new(c)
}