use once_cell;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum RedisCommand {
    ClientList,
    InfoServer,
    InfoCpu,
    InfoStat,
    InfoMemory,
    DbSize,
    InfoClients,
    InfoCommandStats,
    InfoKeySpace,
    InfoReplication,
    Ping,
    GetDatabaseCount,
    GetLimitClientCount,
    GetRedisLimitMemorySize,
    GetClusterGenKeySlotSize,
    GetClusterNodes,
    GetMemoryUsageFromKey,
}
static REIDS_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<RedisCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut REIDS_COMMANDLINE_MAP_internal = HashMap::new();
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::ClientList, "client list");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoServer, "info server");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoCpu, "info cpu");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoStat, "info stats");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoMemory, "info memeory");
        REIDS_COMMANDLINE_MAP_internal.insert(
            RedisCommand::DbSize,
            "eval \"redis.call('select',ARGV[1] ); return redis.call('dbsize')\" 0 ?",
        );
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoClients, "info clients");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoCommandStats, "info commandstats");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoKeySpace, "info keyspace");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::InfoReplication, "info replication");
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::Ping, "ping");
        REIDS_COMMANDLINE_MAP_internal
            .insert(RedisCommand::GetDatabaseCount, "config get databases");
        REIDS_COMMANDLINE_MAP_internal
            .insert(RedisCommand::GetLimitClientCount, "config get maxclients");
        REIDS_COMMANDLINE_MAP_internal.insert(
            RedisCommand::GetRedisLimitMemorySize,
            "config get maxmemory",
        );
        REIDS_COMMANDLINE_MAP_internal.insert(
            RedisCommand::GetClusterGenKeySlotSize,
            "cluster countkeysinslot",
        );
        REIDS_COMMANDLINE_MAP_internal.insert(RedisCommand::GetClusterNodes, "cluster nodes");
        REIDS_COMMANDLINE_MAP_internal
            .insert(RedisCommand::GetMemoryUsageFromKey, "MEMORY USAGE ?");
        REIDS_COMMANDLINE_MAP_internal
    });
