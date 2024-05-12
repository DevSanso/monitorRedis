use std::collections::HashMap;

use std::time::Duration;

use crate::threads::WorkerFn;

mod client_list_worker;


pub fn make_sec_worker() -> HashMap<String,(Duration, WorkerFn)> {
    let mut m : HashMap<String,(Duration, WorkerFn)> = HashMap::new();

    m.insert(String::from("ClientList"), (Duration::from_secs(5), &client_list_worker::client_list_worker));

    m
}

pub fn make_min_worker() -> HashMap<String,(Duration, WorkerFn)> {
    let mut m : HashMap<String,(Duration, WorkerFn)> = HashMap::new();

    m.insert(String::from("ClientList"), (Duration::from_secs(5), &client_list_worker::client_list_worker));

    m
}