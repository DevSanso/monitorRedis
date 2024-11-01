use once_cell;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, std::fmt::Debug, Clone)]
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
    GetClusterGenKeySlotSize,
    GetClusterNodes,
    GetAllConfig,
    GetMemoryKeyUsage3000Range,
}
impl std::fmt::Display for RedisCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
        reids_commandline_map_internal.insert(
            RedisCommand::GetClusterGenKeySlotSize,
            "cluster countkeysinslot",
        );
        reids_commandline_map_internal.insert(RedisCommand::GetClusterNodes, "cluster nodes");
        reids_commandline_map_internal.insert(RedisCommand::GetAllConfig, "config get *");
        reids_commandline_map_internal.insert(RedisCommand::GetMemoryKeyUsage3000Range,"eval \"local scan_val = redis.call('scan', ARGV[1], 'count', 3000);local ks = scan_val[2];local ks_usage = {};local idx = 1;local usage = 0; for k, value in pairs(ks) do local reply = redis.pcall('memory','usage', value); local expired = redis.call('ttl', value); ks_usage[idx] = {value, reply, expired };idx = idx + 1; end;  return {scan_val[1],ks_usage}\" 0 ? ");
        reids_commandline_map_internal
    });
#[derive(Eq, PartialEq, Hash, std::fmt::Debug, Clone)]
pub enum CollectCommand {
    RedisClientList,
    CommonCpu,
    RedisInfoStat,
    RedisDbSize,
    RedisInfoCommandStats,
    CommonConfigAll,
    RedisPingUpdate,
    RedisKeySpace,
    RedisSyncClusterNodes,
    RedisInsertClusterNodesPing,
    RedisDeleteClusterNodes,
    RedisInsertKeyUsageTopTenHundred,
    RedisInsertInfoKeySpace,
    RedisInsertInfoMemory,
}
impl std::fmt::Display for CollectCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub static COLLECT_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<CollectCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut collect_commandline_map_internal = HashMap::new();
        collect_commandline_map_internal.insert(CollectCommand::RedisClientList," INSERT INTO redis_client_list   (server_id, collect_time, id, addr, fd, name, age, idle, flags, db, sub, psub, multi, qbuf, qbuf_free, obl, oll, omem, events, cmd, \"user\")   VALUES ($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20) ");
        collect_commandline_map_internal.insert(CollectCommand::CommonCpu," INSERT INTO comm_cpu (   server_id,   collect_time,   cpu_user  cpu_sys  cpu_idle ) VALUES ( $1, now(), $2, $3, $4) ");
        collect_commandline_map_internal.insert(CollectCommand::RedisInfoStat," INSERT INTO redis_info_stats (   server_id,   collect_time,   total_connections_received,   total_commands_processed,   instantaneous_ops_per_sec,   total_net_input_bytes,   total_net_output_bytes,   instantaneous_input_kbps,   instantaneous_output_kbps,   rejected_connections,   sync_full,   sync_partial_ok,   sync_partial_err,   expired_keys,   evicted_keys,   keyspace_hits,   keyspace_misses,   pubsub_channels,   pubsub_patterns,   latest_fork_usec,   migrate_cached_sockets,   slave_expires_tracked_keys,   active_defrag_hits,   active_defrag_misses,   active_defrag_key_hits,   active_defrag_key_misses   ) VALUES ($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) ");
        collect_commandline_map_internal.insert(CollectCommand::RedisDbSize," INSERT INTO redis_dbsize (   server_id,   collect_time,   db_size  ) VALUES ( $1, now(), $2) ");
        collect_commandline_map_internal.insert(CollectCommand::RedisInfoCommandStats," INSERT INTO redis_info_commandstats(   server_id,   collect_time,   cmd,   calls,   usec,   usec_per_call  ) VALUES ( $1, now(), $2, $3, $4, $5) ");
        collect_commandline_map_internal.insert(CollectCommand::CommonConfigAll," INSERT INTO comm_config_all(   server_id,   sync_time,   name,   value )  VALUES (   $1, now(), $2, $3 )  ON CONFLICT(server_id, name, sync_time)   DO UPDATE   sync_time = now(),   value = $3 ");
        collect_commandline_map_internal.insert(CollectCommand::RedisPingUpdate," INSERT INTO redis_ping_status(   server_id,   sync_time,   status )  VALUES (   $1, now(), $2)  ON CONFLICT(server_id)   DO UPDATE   sync_time = now(),   status = $2");
        collect_commandline_map_internal.insert(CollectCommand::RedisKeySpace," INSERT INTO redis_key_space(   server_id,   collect_time,   db_name,   keys   expires,   avg_ttl   )   VALUES( $1, now(), $2, $3, $4, $5 ) ");
        collect_commandline_map_internal.insert(CollectCommand::RedisSyncClusterNodes," INSERT INTO redis_cluster_nodes (   server_id,   sync_time,   node_id,   ip,   port,   cluster_port,   type,   master_node,   ping_epoch,   connected_state,   slots )   VALUES($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10)   ON CONFLICT(server_id, node_id, ip) DO UPDATE SET   port = $4,   cluster_port = $5,   type = $6,   master_node = $7,   ping_epoch = $8,   connected_state = $9,   slots = $10 ");
        collect_commandline_map_internal.insert(CollectCommand::RedisInsertClusterNodesPing," INSERT INTO redis_cluster_nodes_ping (   server_id,   sync_time,   node_id,   ping_send,   ping_recv )   VALUES($1, now(), $2, $3, $4 ) ");
        collect_commandline_map_internal.insert(CollectCommand::RedisDeleteClusterNodes,"DELETE FROM redis_cluster_nodes where now() - sync_time > '400 seconds' interval and server_id = $1 ");
        collect_commandline_map_internal.insert(CollectCommand::RedisInsertKeyUsageTopTenHundred,"INSERT INTO redis_key_usage_mem(server_id, collect_time, key_name, usage_byte, expired_sec) VALUES($1, now(), $2, $3, $4)");
        collect_commandline_map_internal.insert(CollectCommand::RedisInsertInfoKeySpace,"INSERT INTO redis_info_keyspace(server_id, collect_time, db_name, expires, avg_ttl) VALUES($1, now(), $2, $3, $4)");
        collect_commandline_map_internal.insert(CollectCommand::RedisInsertInfoMemory,"INSERT INTO redis_info_memory(   server_id,   collect_time,   used_memory,   used_memory_rss,   used_memory_peak   used_memory_overhead,   used_memory_dataset,   allocator_allocated,   used_memory_lua,   used_memory_scripts,   maxmemory,   maxmemory_policy,   mem_clients_slaves,   mem_clients_normal,   mem_aof_buffer,   mem_allocator   ) VALUES($1, now(), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)");
        collect_commandline_map_internal
    });
#[derive(Eq, PartialEq, Hash, std::fmt::Debug, Clone)]
pub enum ManageCommand {
    RedisConnInfo,
}
impl std::fmt::Display for ManageCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub static MANAGE_COMMANDLINE_MAP: once_cell::sync::Lazy<HashMap<ManageCommand, &'_ str>> =
    once_cell::sync::Lazy::new(|| {
        let mut manage_commandline_map_internal = HashMap::new();
        manage_commandline_map_internal.insert(ManageCommand::RedisConnInfo," SELECT redis_id, username, password, dbname, ip, port FROM redis_conn where collect_yn = 'Y' ");
        manage_commandline_map_internal
    });
