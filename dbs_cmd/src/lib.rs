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
pub static REIDS_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<RedisCommand, &'_ str>> =
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
#[derive(Eq, PartialEq, Hash)]
pub enum PgCommand {
    ClientList,
    InfoServer,
}
pub static PG_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<PgCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut PG_COMMANDLINE_MAP_internal = HashMap::new();
        PG_COMMANDLINE_MAP_internal.insert(PgCommand::ClientList," INSERT INTO redis_client_list   (link_key, id, addr, fd, name, age, idle, flags, db, sub, psub, multi, qbuf, qbuf_free, obl, oll, omem, events, cmd)   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) ");
        PG_COMMANDLINE_MAP_internal.insert(PgCommand::InfoServer," INSERT INTO redis_info_server (   redis_version,   redis_mode,   os,   arch_bits,   multiplexing_api,   atomicvar_api,   gcc_version,   process_id,   run_id,   tcp_port,   uptime_in_seconds,   uptime_in_days,   hz,   lru_clock,   executable,   config_file  ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) ");
        PG_COMMANDLINE_MAP_internal
    });
