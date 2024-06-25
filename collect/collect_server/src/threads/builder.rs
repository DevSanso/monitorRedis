use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crate::threads::executor::RedisThreadExecutor;

use dbs::sqlite_pool::SqlitePool;
use dbs::pg_pool::PgPool;
use crate::typed::*;
use thread_pool::TPool;

pub struct RedisExectorBulider {
    pg_pool : Option<Arc<Mutex<PgPool>>>,
    sqlite_pool : Option<Arc<Mutex<SqlitePool>>>,
    workers : HashMap<&'static str,(Duration, WorkerFn)>,
    name : &'static str,
    alloc_size : usize,
    redis_single_conn_fn : Option<SelectRedisConnFn>
}


impl RedisExectorBulider {
    pub fn new() -> Self {
        RedisExectorBulider {
            pg_pool : None,
            sqlite_pool : None,
            workers : HashMap::new(),
            name : "",
            alloc_size : 0,
            redis_single_conn_fn : None
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

    pub fn set_redis_select_fn(mut self, select_fn : SelectRedisConnFn) -> Self {
        self.redis_single_conn_fn = Some(select_fn);
        self
    }

    pub fn register_pg(mut self, pg_pool : &'_ Arc<Mutex<PgPool>>) -> Self {
        self.pg_pool = Some(Arc::clone(pg_pool));
        self
    }
    pub fn register_sqlite(mut self, sqlite_pool : &'_ Arc<Mutex<SqlitePool>>) -> Self {
        self.sqlite_pool = Some(Arc::clone(sqlite_pool));
        self
    }
    pub fn register_workers(mut self, w : HashMap<&'static str, (Duration, WorkerFn)>) -> Self {
        self.workers = w;
        self
    }

    pub fn build(self) -> RedisThreadExecutor {
        let worker_keys : Vec<&'static str> = {
            let keys = self.workers.keys();
            let key_len = keys.len();

            keys.fold(Vec::with_capacity(key_len), |mut acc, x| {
                acc.push(*x);
                acc
            })
        };

        RedisThreadExecutor {
            collect_pool : self.pg_pool.unwrap(),
            info_pool : self.sqlite_pool.unwrap(),
            single_conn_fn : self.redis_single_conn_fn.unwrap(),
            worker_fn_list : self.workers,
            redis_pools : HashMap::new(),
            thread_pool : TPool::new(self.name, self.alloc_size),
            redis_worker_flags : HashMap::new(),
            run_workers_list : None,
            woker_fn_keys : worker_keys
        }
    }
}