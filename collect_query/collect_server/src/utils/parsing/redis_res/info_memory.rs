use std::{error::Error};

use core::utils::parsing::common::*;

use core::utils_new_error;
use core::utils_inherit_error;
use core::errs::data::CantMappingKeyError;

#[derive(Default, Debug)]
pub struct InfoMemory {
    pub used_memory: i64,
    pub used_memory_rss: i64,
    pub used_memory_peak: i64,
    pub used_memory_peak_perc: f64,
    pub used_memory_overhead: i64,
    pub used_memory_startup: i64,
    pub used_memory_dataset: i64,
    pub used_memory_dataset_perc: f64,
    pub allocator_allocated: i64,
    pub allocator_active: i64,
    pub allocator_resident: i64,
    pub total_system_memory: i64,
    pub used_memory_lua: i64,
    pub used_memory_scripts: i64,
    pub number_of_cached_scripts: i64,
    pub maxmemory: i64,
    pub maxmemory_human: f64,
    pub maxmemory_policy: String,
    pub allocator_frag_ratio: f64,
    pub allocator_frag_bytes: i64,
    pub allocator_rss_ratio: f64,
    pub allocator_rss_bytes: i64,
    pub rss_overhead_ratio: f64,
    pub rss_overhead_bytes: i64,
    pub mem_fragmentation_ratio: f64,
    pub mem_fragmentation_bytes: i64,
    pub mem_not_counted_for_evict: i64,
    pub mem_replication_backlog: i64,
    pub mem_clients_slaves: i64,
    pub mem_clients_normal: i64,
    pub mem_aof_buffer: i64,
    pub mem_allocator: String,
    pub active_defrag_running: i64,
    pub lazyfree_pending_objects: i64,
}

fn mapping_info_mem(r : &mut InfoMemory, raw_data : &'_ str) -> Result<(), Box<dyn Error>> {
    const UNSUPPORT_KEYS : &'static [&'static str]= &["lazyfreed_objects"];
    
    let s = split_colon_tuple(raw_data)?;

    match s.0{
        "used_memory" => r.used_memory = s.1.trim().parse()?,
        "used_memory_rss" => r.used_memory_rss = s.1.trim().parse()?,
        "used_memory_peak" => r.used_memory_peak = s.1.trim().parse()?,
        "used_memory_peak_perc" => r.used_memory_peak_perc = s.1.trim().parse()?,
        "used_memory_overhead" => r.used_memory_overhead = s.1.trim().parse()?,
        "used_memory_startup" => r.used_memory_startup = s.1.trim().parse()?,
        "used_memory_dataset" => r.used_memory_dataset = s.1.trim().parse()?,
        "used_memory_dataset_perc" => r.used_memory_dataset_perc = s.1.trim().parse()?,
        "allocator_allocated" => r.allocator_allocated = s.1.trim().parse()?,
        "allocator_active" => r.allocator_active = s.1.trim().parse()?,
        "allocator_resident" => r.allocator_resident = s.1.trim().parse()?,
        "total_system_memory" => r.total_system_memory = s.1.trim().parse()?,
        "used_memory_lua" => r.used_memory_lua = s.1.trim().parse()?,
        "used_memory_scripts" => r.used_memory_scripts = s.1.trim().parse()?,
        "number_of_cached_scripts" => r.number_of_cached_scripts = s.1.trim().parse()?,
        "maxmemory" => r.maxmemory = s.1.trim().parse()?,
        "rss_overhead_ratio" => r.rss_overhead_ratio = s.1.trim().parse()?,
        "maxmemory_human" => r.maxmemory_human = s.1.trim().parse()?,
        "maxmemory_policy" => r.maxmemory_policy = s.1.to_string(),
        "allocator_frag_ratio" => r.allocator_frag_ratio = s.1.trim().parse()?,
        "allocator_frag_bytes" => r.allocator_frag_bytes = s.1.trim().parse()?,
        "rss_overhead_bytes" => r.rss_overhead_bytes = s.1.trim().parse()?,
        "allocator_rss_ratio" => r.allocator_rss_ratio = s.1.trim().parse()?,
        "allocator_rss_bytes" => r.allocator_rss_bytes = s.1.trim().parse()?,
        "mem_fragmentation_ratio" => r.mem_fragmentation_ratio = s.1.trim().parse()?,
        "mem_fragmentation_bytes" => r.mem_fragmentation_bytes = s.1.trim().parse()?,
        "mem_not_counted_for_evict" => r.mem_not_counted_for_evict = s.1.trim().parse()?,
        "mem_replication_backlog" => r.mem_replication_backlog = s.1.trim().parse()?,
        "mem_clients_slaves" => r.mem_clients_slaves = s.1.trim().parse()?,
        "mem_clients_normal" => r.mem_clients_normal = s.1.trim().parse()?,
        "mem_aof_buffer" => r.mem_aof_buffer = s.1.trim().parse()?,
        "mem_allocator" => r.mem_allocator = s.1.to_string(),
        "active_defrag_running" => r.active_defrag_running = s.1.trim().parse()?,
        "lazyfree_pending_objects" => r.lazyfree_pending_objects = s.1.trim().parse()?,
        key if UNSUPPORT_KEYS.contains(&key) => {
            log::debug!("not support {} this client list data", key);
        },
        _ => {
            if !s.0.contains("human") {
                return utils_new_error!(data, CantMappingKeyError, s.0)
            } 
        }
    }

    Ok(())

}



pub fn parsing_info_memory(res : String) -> Result<InfoMemory, Box<dyn Error>> {
    let s = res.as_str();
    let mut o = InfoMemory::default();

    for line in s.split("\n").skip(1) {
        if line == "" {continue;}
        let ret = mapping_info_mem(&mut o, line);

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

