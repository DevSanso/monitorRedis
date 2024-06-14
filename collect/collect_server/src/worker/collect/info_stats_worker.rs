use std::error::Error;

use dbs::{self, pg_pool::PgUploader};
use dbs_cmd;

use parsing::redis_res::parsing_info_stat;
use dbs::utils::make_pg_numeric;

pub fn info_stats_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::InfoStat).unwrap();
    let result = redis_conn.run_command(cmd, &[])?;

    let c = parsing_info_stat(result)?;

    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InfoStat).unwrap();
    pg_conn.execute(&pg_query, &[&link_key,
        &c.total_connections_received,
        &c.total_commands_processed,
        &c.instantaneous_ops_per_sec,
        &c.total_net_input_bytes,
        &c.total_net_output_bytes,
        &make_pg_numeric(c.instantaneous_input_kbps),
        &make_pg_numeric(c.instantaneous_output_kbps),
        &c.rejected_connections,
        &c.sync_full,
        &c.sync_partial_ok,
        &c.sync_partial_err,
        &c.expired_keys,
        &c.evicted_keys,
        &c.keyspace_hits,
        &c.keyspace_misses,
        &c.pubsub_channels,
        &c.pubsub_patterns,
        &c.latest_fork_usec,
        &c.migrate_cached_sockets,
        &c.slave_expires_tracked_keys,
        &c.active_defrag_hits,
        &c.active_defrag_misses,
        &c.active_defrag_key_hits,
        &c.active_defrag_key_misses])?;
    Ok(())
}