use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::error::Error;
use std::fmt::Display;
use std::time::Duration;

use chrono::prelude::*;

use dbs::redis_pool::{RedisPool, RedisRequester};
use dbs::pg_pool::{PgPool, PgUploader};
use log::*;
use core::structure::pool::PoolItem;

pub type WorkerFn = &'static (dyn Fn(u32, &'_ mut dbs::redis_pool::RedisRequester, &'_ mut dbs::pg_pool::PgUploader) -> Result<(),Box<dyn Error>> + Send + Sync);

pub struct ThreadExecuter {
    unqiue_id : u32,
    redis_p : Arc<Mutex<RedisPool>>,
    pg_p : Arc<Mutex<PgPool>>,
    control_thread_handle : Option<JoinHandle<Result<(), String>>>,

    mapping_func : HashMap<String,(Duration, WorkerFn)>
}


struct ThreadArgs {
    pub state : Arc<Mutex<bool>>,
    pub pg_p : Arc<Mutex<PgPool>>,
    pub redis_p : Arc<Mutex<RedisPool>>,
    pub unqiue_id : u32,
    pub worker_name : String,
    pub worker_fn : WorkerFn
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

    fn make_func_state_map(ks : Keys<String,(Duration, WorkerFn)>) -> HashMap<String, Arc<Mutex<bool>>> {
        ks.fold(HashMap::new(), |mut acc ,x | {
            acc.insert(x.clone(), Arc::new(Mutex::new(false)));
            acc
        })
    }

    fn run_worker(worker_name : &'_ str, mut r : PoolItem<'_, RedisRequester>, mut p : PoolItem<'_, PgUploader>, unqiue_id : u32, state_used : Arc<Mutex<bool>>, fun : WorkerFn) {
        
        {
            let g_used = state_used.lock().unwrap();
            if *g_used == true { 
                log::info!("run_worker[{}]- {} - already running ", unqiue_id, worker_name);
                return;
             }
        }
        
        let r_ref = &mut r;
        let p_ref = &mut p;
        
        {
            let mut g_used = state_used.lock().unwrap();
            *g_used = true;
        }

        let res = fun(unqiue_id, r_ref.get_value(), p_ref.get_value());
        if res.is_err() {
            log::error!("run_worker - {} error : {}", worker_name, res.err().unwrap().to_string());
        }

        {
            let mut g_used = state_used.lock().unwrap();
            *g_used = false;
        }
    }

    fn spawn_worker_thread(args : ThreadArgs) -> Result<(), Box<dyn Error>> {
        let mut redis_g = args.redis_p.lock().unwrap();
        let redis_conn = redis_g.get()?;

        let mut pg_g = args.pg_p.lock().unwrap();
        let pg_conn = pg_g.get()?;

        Self::run_worker(args.worker_name.as_str(), redis_conn, pg_conn, args.unqiue_id, args.state, args.worker_fn);

        Ok(())
    }

    fn thread_entry<'a>(unqiue_id : u32, redis_p : Arc<Mutex<RedisPool>>, pg_p : Arc<Mutex<PgPool>>, mapping : HashMap<String,(Duration, WorkerFn)>)  -> Result<(),String>{
        let used_states = Self::make_func_state_map(mapping.keys());
        thread::scope(|s| {
            loop {
                {
                    let n = Local::now();
                    let now_sec = n.second();
                    
                    for worker in &mapping {
                        let sec = worker.1.0.as_secs();
                        if now_sec % (sec as u32) != 0 { continue; }

                        let thr_redis_r = Arc::clone(&redis_p);
                        let thr_pg_p = Arc::clone(&pg_p);
                        let state_lock = used_states.get(worker.0).unwrap();
                        let state_lock_clone = Arc::clone(state_lock);
                        let work_name = worker.0.clone();

                        let builder = thread::Builder::new();
                        let _ = builder
                            .name(format!("{} - {}", unqiue_id, &work_name))
                            .stack_size(1024 * 512)
                            .spawn_scoped(s, move || {
                                let args = ThreadArgs {
                                    redis_p : thr_redis_r,
                                    pg_p : thr_pg_p,
                                    state : state_lock_clone,
                                    unqiue_id,
                                    worker_name : work_name,
                                    worker_fn : worker.1.1
                                };
                                let ret = Self::spawn_worker_thread(args);
                                if ret.is_err() {
                                    log::error!("thread_entry[{}] - {} - {}",unqiue_id, worker.0, ret.err().unwrap().to_string());
                                }
                        });
                    }
    
                }
                thread::sleep(Duration::from_millis(1000));
            }
        });

        loop {
            thread::sleep(Duration::from_secs(100));
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