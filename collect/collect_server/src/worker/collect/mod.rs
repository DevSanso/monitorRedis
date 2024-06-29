use std::collections::HashMap;

use std::time::Duration;

use crate::typed::WorkerFn;

mod client_list_worker;
mod info_cpu_worker;
mod info_stats_worker;
mod db_size_worker;
mod info_commandstats_worker;
mod config_get_all_worker;

macro_rules! register_worker_list {
    ($m : expr, $name: expr, $interval : expr, $func : expr) => {
        $m.insert($name, (Duration::from_secs($interval), &$func));
    };
}

pub fn make_one_collect_worker() -> HashMap<&'static str, (Duration, WorkerFn)> {
    let mut m : HashMap<&'static str,(Duration, WorkerFn)> = HashMap::new();

    //register_worker_list!(m, "ClientList", 10, client_list_worker::client_list_worker);
    //register_worker_list!(m, "InfoCpu", 30, info_cpu_worker::info_cpu_worker);
    //register_worker_list!(m, "InfoStat", 60, info_stats_worker::info_stats_worker);
    //register_worker_list!(m, "DBSize", 60, db_size_worker::db_size_worker);
    register_worker_list!(m, "InfoCommandStats", 2, info_commandstats_worker::info_commandstats_worker);
    //register_worker_list!(m, "ConfigAll", 3600, config_get_all_worker::client_list_worker);
    
    m
}
