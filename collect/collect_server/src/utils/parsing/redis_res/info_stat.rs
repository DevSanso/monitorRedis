use std::error::Error;

use core::utils::parsing::common::*;
use core::{errs::data::CantMappingKeyError, utils_new_error, utils_inherit_error};
/*
# CPU
used_cpu_sys:1.944891
used_cpu_user:1.398170
used_cpu_sys_children:0.000000
used_cpu_user_children:0.000000


*/
#[derive(Default,Debug)]
pub struct InfoStat {
    pub total_connections_received : i64,
    pub total_commands_processed : i64,
    pub instantaneous_ops_per_sec : i64,
    pub total_net_input_bytes : i64,
    pub total_net_output_bytes : i64,
    pub instantaneous_input_kbps : f64,
    pub instantaneous_output_kbps : f64,
    pub rejected_connections : i64,
    pub sync_full : i64,
    pub sync_partial_ok : i64,
    pub sync_partial_err : i64,
    pub expired_keys : i64,
    pub expired_stale_perc : f64,
    pub expired_time_cap_reached_count : i64,
    pub evicted_keys : i64,
    pub keyspace_hits : i64,
    pub keyspace_misses : i64,
    pub pubsub_channels : i64,
    pub pubsub_patterns : i64,
    pub latest_fork_usec : i64,
    pub migrate_cached_sockets : i64,
    pub slave_expires_tracked_keys : i64,
    pub active_defrag_hits : i64,
    pub active_defrag_misses : i64,
    pub active_defrag_key_hits : i64,
    pub active_defrag_key_misses : i64
}

#[inline]
fn mapping_info_stat(r : &mut InfoStat, raw_data : &'_ str) -> Result<(), Box<dyn Error>> {
    let s = split_colon_tuple(raw_data)?;

    match s.0.as_str() {
        "total_connections_received" => r.total_connections_received = s.1.as_str().trim().parse()?,
        "total_commands_processed" => r.total_commands_processed = s.1.as_str().trim().parse()?,
        "total_net_input_bytes" => r.total_net_input_bytes = s.1.as_str().trim().parse()?,
        "total_net_output_bytes" => r.total_net_output_bytes = s.1.as_str().trim().parse()?,
        "instantaneous_input_kbps" => r.instantaneous_input_kbps = s.1.as_str().trim().parse()?,
        "instantaneous_output_kbps" => r.instantaneous_output_kbps = s.1.as_str().trim().parse()?,
        "rejected_connections" => r.rejected_connections = s.1.as_str().trim().parse()?,
        "sync_full" => r.sync_full = s.1.as_str().trim().parse()?,
        "sync_partial_ok" => r.sync_partial_ok = s.1.as_str().trim().parse()?,
        "sync_partial_err" => r.sync_partial_err = s.1.as_str().trim().parse()?,
        "expired_keys" => r.expired_keys = s.1.as_str().trim().parse()?,
        "expired_stale_perc" => r.expired_stale_perc = s.1.as_str().trim().parse()?,
        "expired_time_cap_reached_count" => r.expired_time_cap_reached_count = s.1.as_str().trim().parse()?,
        "evicted_keys" => r.evicted_keys = s.1.as_str().trim().parse()?,
        "keyspace_hits" => r.keyspace_hits = s.1.as_str().trim().parse()?,
        "keyspace_misses" => r.keyspace_misses = s.1.as_str().trim().parse()?,
        "pubsub_channels" => r.pubsub_channels = s.1.as_str().trim().parse()?,
        "pubsub_patterns" => r.pubsub_patterns = s.1.as_str().trim().parse()?,
        "latest_fork_usec" => r.latest_fork_usec = s.1.as_str().trim().parse()?,
        "migrate_cached_sockets" => r.migrate_cached_sockets = s.1.as_str().trim().parse()?,
        "slave_expires_tracked_keys" => r.slave_expires_tracked_keys = s.1.as_str().trim().parse()?,
        "active_defrag_hits" => r.active_defrag_hits = s.1.as_str().trim().parse()?,
        "active_defrag_misses" => r.active_defrag_misses = s.1.as_str().trim().parse()?,
        "active_defrag_key_hits" => r.active_defrag_key_hits = s.1.as_str().trim().parse()?,
        "active_defrag_key_misses" => r.active_defrag_key_misses = s.1.as_str().trim().parse()?,
        "instantaneous_ops_per_sec" => r.instantaneous_ops_per_sec = s.1.as_str().trim().parse()?,
        _ => return utils_new_error!(data, CantMappingKeyError, s.0.as_str())
    }

    Ok(())
}

pub fn parsing_info_stat(res : String) -> Result<InfoStat, Box<dyn Error>> {
    let s = res.as_str();
    let mut o = InfoStat::default();

    for line in s.split("\n").skip(1) {
        if line == "" {continue;}
        let ret = mapping_info_stat(&mut o, line);

        if ret.is_err() {
            let err = ret.err().unwrap();
            if !err.is::<CantMappingKeyError>() {
                return utils_inherit_error!(data, GetDataCastError, "", err);
            }
            return Err(err);
        }
    }

    Ok(o)
}