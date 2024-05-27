use std::collections::HashMap;

use std::time::Duration;

use crate::threads::WorkerFn;

mod client_list_worker;
mod info_cpu_worker;
mod info_stats_worker;


pub fn make_sec_worker() -> HashMap<String,(Duration, WorkerFn)> {
    let mut m : HashMap<String,(Duration, WorkerFn)> = HashMap::new();

    m.insert(String::from("ClientList"), (Duration::from_secs(5), &client_list_worker::client_list_worker));
    m.insert(String::from("InfoCpu"), (Duration::from_secs(30), &info_cpu_worker::info_cpu_worker));
    m.insert(String::from("InfoStat"), (Duration::from_secs(60), &info_stats_worker::info_stats_worker));
    
    m
}
