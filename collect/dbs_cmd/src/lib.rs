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
    GetDbSizeSelf,
    InfoClients,
    InfoCommandStats,
    InfoKeySpace,
    InfoReplication,
    GetDatabaseCount,
    GetLimitClientCount,
    GetRedisLimitMemorySize,
    GetClusterGenKeySlotSize,
    GetClusterNodes,
    GetMemoryUsageFromKey,
    GetAllConfig,
    GetMemoryKeyUsage3000Range,
}
pub static REIDS_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<RedisCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut reids_commandline_map_internal = HashMap::new();
        reids_commandline_map_internal.insert(RedisCommand::ClientList, "client list");
        reids_commandline_map_internal.insert(RedisCommand::InfoServer, "info server");
        reids_commandline_map_internal.insert(RedisCommand::InfoCpu,"eval \"local cpu = redis.call('info', 'cpu');local n = redis.call('info', 'server'); local ret = {};ret[1] = string.match(n, 'uptime_in_seconds:(%d+)');ret[2]=cpu;return ret\" 0");
        reids_commandline_map_internal.insert(RedisCommand::InfoStat, "info stats");
        reids_commandline_map_internal.insert(RedisCommand::InfoMemory, "info memeory");
        reids_commandline_map_internal.insert(
            RedisCommand::DbSize,
            "eval \"redis.call('select',ARGV[1]); return redis.call('dbsize')\" 0 ?",
        );
        reids_commandline_map_internal.insert(RedisCommand::GetDbSizeSelf, "dbsize");
        reids_commandline_map_internal.insert(RedisCommand::InfoClients, "info clients");
        reids_commandline_map_internal.insert(RedisCommand::InfoCommandStats, "info commandstats");
        reids_commandline_map_internal.insert(RedisCommand::InfoKeySpace, "info keyspace");
        reids_commandline_map_internal.insert(RedisCommand::InfoReplication, "info replication");
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
        reids_commandline_map_internal.insert(RedisCommand::GetAllConfig, "config get *");
        reids_commandline_map_internal.insert(RedisCommand::GetMemoryKeyUsage3000Range,"eval \"local scan_val = redis.call('scan', ARGV[1], 'count', 3000);local ks = scan_val[2];local ks_usage = {};local idx = 1;local usage = 0; for k, value in pairs(ks) do local reply = redis.pcall('memory','usage', value); local expired = redis.call('ttl', value); ks_usage[idx] = {value, reply, expired };idx = idx + 1; end;  return {scan_val[1],ks_usage}\" 0 ? ");
        reids_commandline_map_internal
    });
#[derive(Eq, PartialEq, Hash)]
pub enum PgCommand {
    ClientList,
    InfoCpu,
    InfoStat,
    DbSize,
    InfoCommandStats,
    ConfigAll,
    PingUpdate,
    KeySpace,
    SyncClusterNodes,
    InsertClusterNodesPing,
    DeleteClusterNodes,
    InsertKeyUsageTopTenHundred,
}
pub static PG_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<PgCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut pg_commandline_map_internal = HashMap::new();
        pg_commandline_map_internal.insert(PgCommand::ClientList," INSERT INTO redis_client_list   (link_key, collect_time, id, addr, fd, name, age, idle, flags, db, sub, psub, multi, qbuf, qbuf_free, obl, oll, omem, events, cmd, \"user\")   VALUES ($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20) ");
        pg_commandline_map_internal.insert(PgCommand::InfoCpu," INSERT INTO redis_info_cpu (   link_key,   collect_time,   used_cpu_sys,   used_cpu_user,   used_cpu_sys_children,   used_cpu_user_children,   uptime  ) VALUES ( $1, now(), $2, $3, $4, $5, $6) ");
        pg_commandline_map_internal.insert(PgCommand::InfoStat," INSERT INTO redis_info_stats (   link_key,   collect_time,   total_connections_received,   total_commands_processed,   instantaneous_ops_per_sec,   total_net_input_bytes,   total_net_output_bytes,   instantaneous_input_kbps,   instantaneous_output_kbps,   rejected_connections,   sync_full,   sync_partial_ok,   sync_partial_err,   expired_keys,   evicted_keys,   keyspace_hits,   keyspace_misses,   pubsub_channels,   pubsub_patterns,   latest_fork_usec,   migrate_cached_sockets,   slave_expires_tracked_keys,   active_defrag_hits,   active_defrag_misses,   active_defrag_key_hits,   active_defrag_key_misses   ) VALUES ($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) ");
        pg_commandline_map_internal.insert(PgCommand::DbSize," INSERT INTO redis_dbsize (   link_key,   collect_time,   db_size  ) VALUES ( $1, now(), $2) ");
        pg_commandline_map_internal.insert(PgCommand::InfoCommandStats," INSERT INTO redis_info_commandstats(   link_key,   collect_time,   cmd,   calls,   usec,   usec_per_call  ) VALUES ( $1, now(), $2, $3, $4, $5) ");
        pg_commandline_map_internal.insert(PgCommand::ConfigAll," INSERT INTO redis_config_all(   link_key,   sync_time,   name,   value )  VALUES (   $1, now(), $2, $3 )  ON CONFLICT(link_key, name, sync_time)   DO UPDATE   sync_time = now(),   value = $3 ");
        pg_commandline_map_internal.insert(PgCommand::PingUpdate," INSERT INTO redis_ping_status(   link_key,   sync_time,   status )  VALUES (   $1, now(), $2)  ON CONFLICT(link_key)   DO UPDATE   sync_time = now(),   status = $2");
        pg_commandline_map_internal.insert(PgCommand::KeySpace," INSERT INTO redis_key_space(   link_key,   collect_time,   db_name,   expires,   avg_ttl   )   VALUES( $1, now(), $2, $3, $4, $5 ) ");
        pg_commandline_map_internal.insert(PgCommand::SyncClusterNodes," INSERT INTO redis_cluster_nodes (   link_key,   sync_time,   node_id,   ip,   port,   cluster_port,   type,   master_node,   ping_epoch,   connected_state,   slots )   VALUES($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10)   ON CONFLICT(link_key, node_id, ip) DO UPDATE SET   port = $4,   cluster_port = $5,   type = $6,   master_node = $7,   ping_epoch = $8,   connected_state = $9,   slots = $10 ");
        pg_commandline_map_internal.insert(PgCommand::InsertClusterNodesPing," INSERT INTO redis_cluster_nodes_ping (   link_key,   sync_time,   node_id,   ping_send,   ping_recv )   VALUES($1, now(), $2, $3, $4 ) ");
        pg_commandline_map_internal.insert(PgCommand::DeleteClusterNodes,"DELETE FROM redis_cluster_nodes where now() - sync_time > '400 seconds' interval and link_key = $1 ");
        pg_commandline_map_internal.insert(PgCommand::InsertKeyUsageTopTenHundred,"INERT INTO redis_key_usage_mem(link_key, collect_time, key_name, usage_byte, expired_sec) VALUES($1, now(), $2, $3, $5)");
        pg_commandline_map_internal
    });
#[derive(Eq, PartialEq, Hash)]
pub enum SQLiteCommand {
    RedisConnInfo,
}
pub static SQLITE_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<SQLiteCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut sqlite_commandline_map_internal = HashMap::new();
        sqlite_commandline_map_internal.insert(SQLiteCommand::RedisConnInfo," SELECT redis_id, username, password, dbname, ip, port FROM redis_conn where collect_yn = 'Y' ");
        sqlite_commandline_map_internal
    });
