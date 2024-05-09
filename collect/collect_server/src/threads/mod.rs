use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::error::Error;
use std::time::Duration;

use chrono::prelude::*;
use log::*;

use dbs::redis_pool::{RedisPool, RedisRequester};
use dbs::pg_pool::{PgPool, PgUploader};
use core::structure::pool::PoolItem;

pub type WorkerFn = &'static (dyn Fn(u32, &'_ mut dbs::redis_pool::RedisRequester, &'_ mut dbs::pg_pool::PgUploader) -> Result<(),Box<dyn Error>> + Send + Sync);

pub struct ThreadExecuter {
    unqiue_id : u32,
    redis_p : Arc<Mutex<RedisPool>>,
    pg_p : Arc<Mutex<PgPool>>,
    control_thread_handle : Option<JoinHandle<Result<(), String>>>,

    mapping_func : HashMap<String,(Duration, WorkerFn)>
}

impl ThreadExecuter {
    pub fn new(unqiue_id : u32, redis_p : RedisPool, global_pg_p : Arc<Mutex<PgPool>>, mapping : HashMap<String,(Duration, WorkerFn)>) -> Self {
        ThreadExecuter {
            unqiue_id,
            redis_p : Arc::new(Mutex::new(redis_p)),
            pg_p : global_pg_p,
            control_thread_handle : None,
            mapping_func : mapping
        }
    }
    fn run_worker<'a>(worker_name : &'a str, mut r : PoolItem<'a, RedisRequester>, mut p : PoolItem<'a, PgUploader>, unqiue_id : u32, fun : WorkerFn) {
        let r_ref = &mut r;
        let p_ref = &mut p;

        let res = fun(unqiue_id, r_ref.get_value(), p_ref.get_value());

        if res.is_err() {
            log::error!("run_worker - {} error : {}", worker_name, res.err().unwrap().to_string());
        }
    }
    fn thread_entry(unqiue_id : u32, redis_p : Arc<Mutex<RedisPool>>, pg_p : Arc<Mutex<PgPool>>, mapping : HashMap<String,(Duration, WorkerFn)>)  -> Result<(),String>{
        loop {
            {
                let n = Local::now();
                let now_sec = n.second();
                
                for worker in &mapping {
                    {
                        let sec = worker.1.0.as_secs();

                        if now_sec % (sec as u32) != 0 { continue; }

                        let mut g_r = redis_p.lock().unwrap();
                        let mut g_p = pg_p.lock().unwrap();
                        

                        let redis_conn = g_r.get();
                        if redis_conn.is_err() { continue; }

                        let pg_conn = g_p.get();
                        if pg_conn.is_err() { continue; }
                        
                        let redis_unwrap = redis_conn.unwrap();
                        let pg_unwrap = pg_conn.unwrap();
                    
                        Self::run_worker(worker.0.as_str(), redis_unwrap, pg_unwrap, unqiue_id, worker.1.1);
                    }  
                }
                let eleped = Local::now().second() - now_sec;

                if eleped > 0 {
                    log::info!("ThreadExecuter::thread_entry - {} - diff over 1 second [ {} ]", unqiue_id, eleped);
                }

            }
            thread::sleep(Duration::from_millis(100));
        }
        //Ok(())
    }

    pub fn auto_no_block_run(&mut self) {
        let clone_redis_p = Arc::clone(&self.redis_p);
        let clone_pg_p = Arc::clone(&self.pg_p);
        let map = self.mapping_func.clone();
        let id = self.unqiue_id;

        let handle = Some(thread::spawn( move || {
            Self::thread_entry(id, clone_redis_p, clone_pg_p, map)
        }));

        self.control_thread_handle = handle;
    }
}