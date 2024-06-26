use std::collections::hash_map::Keys;
use std::hash::Hash;
use std::{collections::HashMap, error::Error};
use std::time::Duration;
use std::sync::{Arc, Mutex};

use chrono::Timelike;

use dbs::sqlite_pool::SqlitePool;
use dbs::{pg_pool::PgPool, redis_pool::RedisPool};
use crate::typed::*;
use thread_pool::{TPool,TItem};
use core::{utils_inherit_error, utils_new_error};

type RedisHashCode = Vec<u8>;
type RedisLinkId = i32;
type RedisConnRunFlag = HashMap<&'static str, Arc<Mutex<bool>>>;

pub struct RedisThreadExecutor {
    pub(super) collect_pool : Arc<Mutex<PgPool>>,
    pub(super) info_pool : Arc<Mutex<SqlitePool>>,
    pub(super) single_conn_fn : SelectRedisConnFn,
    pub(super) worker_fn_list : HashMap<&'static str, (Duration, WorkerFn)>,
    pub(super) redis_pools : HashMap<RedisHashCode, (Arc<Mutex<RedisPool>>, RedisLinkId)>,
    pub(super) redis_worker_flags  : HashMap<RedisHashCode, RedisConnRunFlag>,
    pub(super) thread_pool : TPool<WrapperWorkerArgs>,
    pub(super) run_workers_list : Option<Vec<(RedisHashCode, &'static str)>>,
    pub(super) woker_fn_keys : Vec<&'static str>

}

impl RedisThreadExecutor {
    fn pop_redis_connect_list(&mut self, new_list : &'_ [RedisConnCfg]) {
        let h_ref = &mut self.redis_pools;
        let fn_map = &mut self.redis_worker_flags;
        
        let size = new_list.len();
        let mut delete_list = Vec::with_capacity(size);

        for idx in 0..size {
            let find_redis = h_ref.keys().any(|x| {
                x.as_slice() == new_list[idx].hash_code.as_slice()
            });

            if !find_redis {
                delete_list.push(true);
            }else {
                delete_list.push(false);
            }
        }
        let mut iter = delete_list.iter();
        h_ref.retain(|_,_| {*iter.next().unwrap()});
        iter = delete_list.iter();
        fn_map.retain(|_,_| {*iter.next().unwrap()});
        
    }

    fn push_redis_connect_list(&mut self, new_list : &'_ [RedisConnCfg]) {
        let h_ref = &mut self.redis_pools;
        let fn_map = &mut self.redis_worker_flags;
        let key_list = self.woker_fn_keys.as_slice();
        
        
        let size = new_list.len();
        let mut push_list = Vec::with_capacity(size);

        for idx in 0..size {
            let fined =h_ref.keys().any(|x|{x.as_slice() == new_list[idx].hash_code.as_slice()});
            if !fined {
                let cfg = &new_list[idx].conn_cfg;
                let p = RedisPool::new(cfg.ip.clone(),
                    dbs::utils::create_redis_url(cfg.user.as_str(), cfg.password.as_str(), cfg.ip.as_str(), cfg.port, cfg.db_name));
                push_list.push((p, &new_list[idx].hash_code, new_list[idx].link_id));
            }
        }

        for new_p in push_list {
            h_ref.insert(new_p.1.clone(), (Arc::new(Mutex::new(new_p.0)), new_p.2));
            let hash_keys = key_list.iter().fold(HashMap::new(), |mut acc,x| {
                acc.insert(*x, Arc::new(Mutex::new(false)));
                acc
            }) ;
            fn_map.insert(new_p.1.clone(), hash_keys);
        }
    }

    fn wrapper_fn(arg : Option<WrapperWorkerArgs>) -> Result<(), Box<dyn Error>> {
        if arg.is_none() {
            return utils_new_error!(proc, NoneDataError, "arg is None");
        }

        let arg_some = arg.unwrap();

        let flag = arg_some.flag;

        {
            let mut f_g = flag.lock().unwrap();
            *f_g = true;
        }
        
        let mut redis_p = arg_some.redis_pool.lock().unwrap();
        let mut p_p = arg_some.pg_pool.lock().unwrap();

        {
            let p_c_ret = p_p.get();
            let redis_c_ret = redis_p.get();

            if redis_c_ret.is_err() { 
                let mut f_g = flag.lock().unwrap();
                *f_g = false;
                return utils_inherit_error!(proc, RootError, "redis pool can't get conn", redis_c_ret.err().unwrap());
            }


            if p_c_ret.is_err() { 
                let mut f_g = flag.lock().unwrap();
                *f_g = false;
                return utils_inherit_error!(proc, RootError, "redis pool can't get conn", p_c_ret.err().unwrap());
            }

            let mut redis_c = redis_c_ret.unwrap();
            let mut p_c = p_c_ret.unwrap();

            let id = arg_some.id;
            let func = arg_some.real_worker_fn;
            let ret = func(id, redis_c.get_value(), p_c.get_value());

            {
                let mut f_g = flag.lock().unwrap();
                *f_g = false;
            }
            return ret;
        }
    }

    fn run_conn_worker(&mut self, idles : Vec<(RedisHashCode, &'static str)>) {
        let fns : Vec<TItem<WrapperWorkerArgs>> = idles.iter().fold(Vec::new(), |mut acc, x| {
            let redis_conf = &self.redis_pools[&x.0];
            let redis_work_flag = &self.redis_worker_flags[&x.0][x.1];

            let flag = Arc::clone(redis_work_flag);
            let pg_pool = Arc::clone(&self.collect_pool);
            let redis_pool = Arc::clone(&redis_conf.0);
            let link_id = redis_conf.1;
            let real_fn = self.worker_fn_list[x.1];

            let args = WrapperWorkerArgs {
                flag,
                pg_pool,
                redis_pool,
                id : link_id,
                real_worker_fn : real_fn.1
            };

            acc.push((Some(args), &Self::wrapper_fn));
            acc
        });

        self.thread_pool.use_pool_from_vec(fns);

    }
    
    pub fn load_redis_connect_info(&mut self) -> Result<(), Box<dyn Error>>{
        let list = {
            let mut p: std::sync::MutexGuard<SqlitePool> = self.info_pool.lock().unwrap();
            let conn = match p.get() {
                Ok(ok) => ok,
                Err(err) => return utils_inherit_error!(connection , GetConnectionFailedError, "load_redis_connect_info", err)
            };

            match (self.single_conn_fn)(conn) {
                Ok(ok) => ok,
                Err(err) => return utils_inherit_error!(fetch , GetFailedError, "load_redis_connect_info", err)
            }
        };

        self.pop_redis_connect_list(list.as_slice());
        self.push_redis_connect_list(list.as_slice());
    
        Ok(())
    }

    pub fn update_run_worker(&mut self) {
        let now_sec = chrono::Local::now().second() as u64;

        let mut v = Vec::<(RedisHashCode, &'static str)>::new();

        for redis_p in &self.redis_pools {
            let hash = redis_p.0.clone();
            let worker_list= self.redis_worker_flags[&hash].iter();
            for item in worker_list {
                let run_g = item.1.lock().unwrap();
                if !*run_g {
                    let is_interval = now_sec % self.worker_fn_list[item.0].0.as_secs() == 0;
                    if is_interval {
                        v.push((hash.clone(), item.0));
                    }
                }
            }
        }

        self.run_workers_list = Some(v);
    }

    pub fn run_worker(&mut self) -> Result<(),Box<dyn Error>>{
        let conn_list = match self.run_workers_list.take() {
            Some(s) => s,
            None => return utils_new_error!(proc, NoneDataError, "ThreadExecute worker list is None, plz call update_run_worker")
        };

        let idle_list = conn_list.iter().fold(Vec::new(),|mut acc,x| {
            let worker = x.1;
            let hash_code = x.0.clone();
            
            {
                
                let run_mutex = self.redis_worker_flags
                    .get_mut(&hash_code)
                    .unwrap()
                    .get_mut(worker)
                    .unwrap();

                let is_run_g = run_mutex.lock().unwrap();
                if !*is_run_g {
                    acc.push((hash_code, worker));
                }
            }
            acc
        });
        self.run_conn_worker(idle_list);
        
        Ok(())
    }
}