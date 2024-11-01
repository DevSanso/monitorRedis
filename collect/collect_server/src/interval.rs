use std::time;

use chrono::Timelike;

use dbs_cmd::RedisCommand;

pub fn get_redis_current_interval() -> Vec<RedisCommand> {
    let now_sec = chrono::Local::now().second() as u64;
    let mut v = Vec::new();

    if now_sec % 5 == 0 {
        v.push(RedisCommand::ClientList);
        v.push(RedisCommand::InfoClients);
    }

    if now_sec % 15 == 0 {
        v.push(RedisCommand::InfoCpu);
        v.push(RedisCommand::InfoMemory);
    }

    if now_sec % 60 == 0 {
        v.push(RedisCommand::InfoKeySpace);
        v.push(RedisCommand::InfoCommandStats);
        v.push(RedisCommand::GetDbSizeSelf);
        v.push(RedisCommand::GetClusterNodes);
        v.push(RedisCommand::InfoServer);
        v.push(RedisCommand::InfoStat);
    }

    if now_sec % 3600 == 0 {
        v.push(RedisCommand::GetAllConfig);
        v.push(RedisCommand::GetMemoryKeyUsage3000Range);
    }

    v
}