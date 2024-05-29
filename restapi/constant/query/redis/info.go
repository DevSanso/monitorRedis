package redis

const (
	InfoCpuQuery = `
	SELECT  
		collect_time, used_cpu_sys, used_cpu_user, used_cpu_sys_children, used_cpu_user_children
	FROM redis_info_cpu WHERE unqiue_id  = ? AND column_name BETWEEN TO_TIMESTAMP('yyyymmddhhmi24ss',?) AND TO_TIMESTAMP('yyyymmddhhmi24ss',?)`

	InfoStatsQuery = `
	SELECT
		link_key,  ,
		collect_time,  ,
		total_connections_received,  ,
		total_commands_processed,  ,
		instantaneous_ops_per_sec,  ,
		total_net_input_bytes,  ,
		total_net_output_bytes,  ,
		instantaneous_input_kbps,  ,
		instantaneous_output_kbps,  ,
		rejected_connections,  ,
		sync_full,  ,
		sync_partial_ok,  ,
		sync_partial_err,  ,
		expired_keys,  ,
		evicted_keys,  ,
		keyspace_hits,  ,
		keyspace_misses,  ,
		pubsub_channels,  ,
		pubsub_patterns,  ,
		latest_fork_usec,  ,
		migrate_cached_sockets,  ,
		slave_expires_tracked_keys,  , 
		active_defrag_hits,  ,
		active_defrag_misses,  ,
		active_defrag_key_hits,  ,
		active_defrag_key_misses  
	FROM
		redis_info_stats
	WHERE 
		unqiue_id  = ? 
		AND column_name = TO_TIMESTAMP('yyyymmddhhmi24ss',?)
	`
)