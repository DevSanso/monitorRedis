use std::{collections::HashMap, error::Error};
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::threads::executor::ThreadExecutor;

use dbs::{pg_pool::PgPool, redis_pool::RedisPool};
use crate::typed::*;
use thread_pool::TPool;
use core::utils_new_error;

pub struct ExectorBulider {
    redis_ps : Vec<(i32, Arc<Mutex<RedisPool>>)>,
    pg_pool : Option<Arc<Mutex<PgPool>>>,

    workers : HashMap<String,(Duration, WorkerFn)>,

    name : &'static str,
    alloc_size : usize
}

impl ExectorBulider {
    pub fn new() -> Self {
        ExectorBulider {
            redis_ps : Vec::new(),
            pg_pool : None,
            workers : HashMap::new(),
            name : "",
            alloc_size : 0
        }
    }
    pub fn set_name(mut self, name : &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn set_alloc_size(mut self, size : usize) -> Self {
        self.alloc_size = size;
        self
    }

    pub fn register_pg(mut self, pg_pool : PgPool) -> Self {
        self.pg_pool = Some(Arc::new(Mutex::new(pg_pool)));
        self
    }

    pub fn register_redis(mut self, idx : i32, redis_pool : RedisPool) -> Self {
        self.redis_ps.push((idx, Arc::new(Mutex::new(redis_pool))));
        self
    }

    pub fn register_worker(mut self, name : String, interval : Duration, f : WorkerFn) -> Self {
        self.workers.insert(name, (interval, f));
        self
    }

    fn make_flags(redis_keys : Vec<i32>, worker_names : Vec<String>) ->HashMap<(i32, String), Arc<Mutex<bool>>> {
        let mut h = HashMap::new();
        
        for idx in redis_keys.iter() {
            for name in worker_names.iter() {
                h.insert((idx.clone(), name.clone()), Arc::new(Mutex::new(false)));
            }
        }

        h
    }

    pub fn build(mut self) -> Result<ThreadExecutor, Box<dyn Error>> {
        let redis_keys = self.redis_ps.iter().fold(Vec::new(), |mut acc : Vec<i32>, x| {
            acc.push(x.0);
            acc
        });

        let worker_kets = self.workers.iter().fold(Vec::new(), |mut acc : Vec<String>, x| {
            acc.push(x.0.clone());
            acc
        }); 

        if self.pg_pool.is_none() {
            return utils_new_error!(proc, NoneDataError, "pg_pool");
        }

        let ret = ThreadExecutor {
            t_pool : TPool::new(self.name, self.alloc_size),
            redis_ps : self.redis_ps,
            pg_pool : self.pg_pool.unwrap(),
            workers : self.workers,
            running_flags : Self::make_flags(redis_keys, worker_kets)
        };

        Ok(ret)

    }
}