mod collect_handle;

use std::error::Error;
use std::collections::HashMap;

use collect_handle::*;

use crate::global::get_redis_global;
use core::utils_new_error;
use dbs_cmd::{RedisCommand, REIDS_COMMANDLINE_MAP, CollectCommand};
use dbs::redis_pool::{RedisPoolAlias, RedisRequester};

pub struct RedisCollector {
    server_id : i32,
    command : RedisCommand
}

fn simple_run_cmd(server_id : i32, conn : &'_ mut RedisRequester, command : &'_ RedisCommand) -> Result<(), Box<dyn Error>>{
    let cmd = REIDS_COMMANDLINE_MAP.get(command).unwrap();
    let res = conn.run_command(cmd, &[])?;
    
    match command {
        RedisCommand::ClientList => client_list_handle(server_id, res),
        RedisCommand::InfoCpu => info_cpu_handle(server_id, res),
        RedisCommand::GetClusterNodes => cluster_nodes_handle(server_id, res),
        RedisCommand::GetAllConfig => config_get_all_handle(server_id, res),
        RedisCommand::GetDbSizeSelf => db_size_handle(server_id, res),
        RedisCommand::InfoStat => info_stats_handle(server_id, res),
        _ => utils_new_error!(proc, CriticalError, format!("unkown command handle : {}", command.to_string()))
    }?;

    Ok(())
}

fn complex_run_cmd(server_id : i32, conn : &'_ mut RedisRequester, command : &'_ RedisCommand) -> Result<(), Box<dyn Error>> {
    let cmd = REIDS_COMMANDLINE_MAP.get(command).unwrap();
    
    match command {
        RedisCommand::GetMemoryKeyUsage3000Range => key_usage_top_ten_hundred_handle(server_id, conn, cmd),
        _ => utils_new_error!(proc, CriticalError, format!("unkown command handle : {}", command.to_string()))
        
    }?;
    Ok(())
}

impl crate::collector::Collector<dbs::redis_pool::RedisRequester, RedisCommand> for RedisCollector {
    fn run_collect(&mut self) -> Result<(), Box<dyn Error>> {
        let mut item = {
            get_redis_global().pools.server_pool.get_owned(())?
        };

        let conn = item.get_value();

        conn.set_app_name(format!("collect-{}", self.command).as_str())?;

        if self.command == RedisCommand::GetMemoryKeyUsage3000Range {
            complex_run_cmd(self.server_id, conn, &self.command)?;
            Ok(())
        }
        else {
            simple_run_cmd(self.server_id, conn, &self.command)?;
            Ok(())
        }
    }
    fn get_cmd(&self) -> RedisCommand {
        self.command.clone()
    }
}

pub fn make_redis_collector(server_id : i32, command : RedisCommand) -> Box<RedisCollector> {
    let c = RedisCollector{server_id, command};
    Box::new(c)
}