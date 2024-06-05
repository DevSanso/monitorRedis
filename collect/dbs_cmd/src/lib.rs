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
        let mut reids_commandline_map_internal = HashMap::new();
        reids_commandline_map_internal.insert(RedisCommand::ClientList, "client list");
        reids_commandline_map_internal.insert(RedisCommand::InfoServer, "info server");
        reids_commandline_map_internal.insert(RedisCommand::InfoCpu, "info cpu");
        reids_commandline_map_internal.insert(RedisCommand::InfoStat, "info stats");
        reids_commandline_map_internal.insert(RedisCommand::InfoMemory, "info memeory");
        reids_commandline_map_internal.insert(
            RedisCommand::DbSize,
            "eval \"redis.call('select',ARGV[1]); return redis.call('dbsize')\" 0 ?",
        );
        reids_commandline_map_internal.insert(RedisCommand::InfoClients, "info clients");
        reids_commandline_map_internal.insert(RedisCommand::InfoCommandStats, "info commandstats");
        reids_commandline_map_internal.insert(RedisCommand::InfoKeySpace, "info keyspace");
        reids_commandline_map_internal.insert(RedisCommand::InfoReplication, "info replication");
        reids_commandline_map_internal.insert(RedisCommand::Ping, "ping");
        reids_commandline_map_internal
            .insert(RedisCommand::GetDatabaseCount, "config get databases");
        reids_commandline_map_internal
            .insert(RedisCommand::GetLimitClientCount, "config get maxclients");
        reids_commandline_map_internal.insert(
            RedisCommand::GetRedisLimitMemorySize,
            "config get maxmemory",
        );
        reids_commandline_map_internal.insert(
            RedisCommand::GetClusterGenKeySlotSize,
            "cluster countkeysinslot",
        );
        reids_commandline_map_internal.insert(RedisCommand::GetClusterNodes, "cluster nodes");
        reids_commandline_map_internal
            .insert(RedisCommand::GetMemoryUsageFromKey, "MEMORY USAGE ?");
        reids_commandline_map_internal
    });
#[derive(Eq, PartialEq, Hash)]
pub enum PgCommand {
    ClientList,
    InfoCpu,
    InfoStat,
    DbSize,
    InfoCommandStats,
}
pub static PG_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<PgCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut pg_commandline_map_internal = HashMap::new();
        pg_commandline_map_internal.insert(PgCommand::ClientList," INSERT INTO redis_client_list   (link_key, collect_time, id, addr, fd, name, age, idle, flags, db, sub, psub, multi, qbuf, qbuf_free, obl, oll, omem, events, cmd)   VALUES ($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) ");
        pg_commandline_map_internal.insert(PgCommand::InfoCpu," INSERT INTO redis_info_cpu (   link_key,   collect_time,   used_cpu_sys,   used_cpu_user,   used_cpu_sys_children,   used_cpu_user_children  ) VALUES ( $1, now(), $2, $3, $4, $5) ");
        pg_commandline_map_internal.insert(PgCommand::InfoStat," INSERT INTO redis_info_stats (   link_key,   collect_time,   total_connections_received,   total_commands_processed,   instantaneous_ops_per_sec,   total_net_input_bytes,   total_net_output_bytes,   instantaneous_input_kbps,   instantaneous_output_kbps,   rejected_connections,   sync_full,   sync_partial_ok,   sync_partial_err,   expired_keys,   evicted_keys,   keyspace_hits,   keyspace_misses,   pubsub_channels,   pubsub_patterns,   latest_fork_usec,   migrate_cached_sockets,   slave_expires_tracked_keys,   active_defrag_hits,   active_defrag_misses,   active_defrag_key_hits,   active_defrag_key_misses   ) VALUES ($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) ");
        pg_commandline_map_internal.insert(PgCommand::DbSize," INSERT INTO redis_dbsize (   link_key,   collect_time,   dbname,   db_size  ) VALUES ( $1, now(), $2, $3) ");
        pg_commandline_map_internal.insert(PgCommand::InfoCommandStats," INSERT INTO redis_info_commandstats(   link_key,   collect_time,   cmd,   calls   usec   usec_per_call  ) VALUES ( $1, now(), $2, $3, $4, $5) ");
        pg_commandline_map_internal
    });
#[derive(Eq, PartialEq, Hash)]
pub enum SQLiteCommand {
    RedisConnInfo,
}
pub static SQLITE_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<SQLiteCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut sqlite_commandline_map_internal = HashMap::new();
        sqlite_commandline_map_internal.insert(
            SQLiteCommand::RedisConnInfo,
            " SELECT redis_id, username, password, dbname, ip, port FROM redis_conn ",
        );
        sqlite_commandline_map_internal
    });
