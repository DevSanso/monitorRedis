use std::{collections::HashMap, error::Error};
use std::time::Duration;
use std::sync::{Arc, Mutex};

use chrono::Timelike;

use dbs::{pg_pool::PgPool, redis_pool::RedisPool};
use crate::typed::*;
use thread_pool::{TPool,TItem};
use crate::errors::*;

pub struct ThreadExecutor {
    pub(super) t_pool : TPool<WrapperWorkerArgs>,
    pub(super) redis_ps : Vec<(i32, Arc<Mutex<RedisPool>>)>,
    pub(super) pg_pool : Arc<Mutex<PgPool>>,

    pub(super) workers : HashMap<String,(Duration, WorkerFn)>,
    pub(super) running_flags : HashMap<(i32, String), Arc<Mutex<bool>>>
}

fn wrapper_worker_fn(arg : Option<WrapperWorkerArgs>) -> Result<(),Box<dyn Error>> {
    if arg.is_none() {
        return Err(Box::new(crate::errors::WrapperNoneArgsError));
    }

    let s = arg.unwrap();

    {
        let mut flag = s.flag.lock().unwrap();
        if *flag == true { return Ok(()); }

        *flag = true;
    }

    let mut g_p = s.pg_pool.lock().unwrap();
    let mut g_redis = s.redis_pool.lock().unwrap();
    let work = s.real_worker_fn;

    {
        let mut post_item = match g_p.get() {
            Ok(ok) => ok,
            Err(e) => {
                {
                    let mut flag = s.flag.lock().unwrap();
                    *flag = false;
                }
                return Err(Box::new(PgPoolGetError(e.to_string())));
            }
        };

        let mut redis_item = match g_redis.get() {
            Ok(ok) => ok,
            Err(e) => {
                {
                    let mut flag = s.flag.lock().unwrap();
                    *flag = false;
                }
                return Err(Box::new(RedisPoolGetError(e.to_string())));
            }
        };

        let pg_uploader = post_item.get_value();
        let redis_requester = redis_item.get_value();

        let ret = work(s.id, redis_requester, pg_uploader);
        
        post_item.restoration();
        redis_item.restoration();
        
        {
            let mut flag = s.flag.lock().unwrap();
            *flag = false;
        }
        
        if let Err(e) = ret {
            return Err(Box::new(WorkerRetureError(e.to_string())));
        }
    }

    Ok(())
}

impl ThreadExecutor {
    pub fn run_workers(&mut self) {
        let now = chrono::Local::now().second() as u64;
        let idx = 0;
        
        for w in self.workers.iter() {
            let key = w.0;
            let interval = w.1.0;
            let worker = w.1.1;

            if now % interval.as_secs() != 0 { continue; }
            
            {
                let mut pool_args : Vec<TItem<WrapperWorkerArgs>> = vec![];
                for redis_p in self.redis_ps.iter() {
                    let flag = Arc::clone(&self.running_flags[&(redis_p.0, key.clone())]);
                    let pg = Arc::clone(&self.pg_pool);
                    let redis = Arc::clone(&redis_p.1);

                    let args = Some(WrapperWorkerArgs {
                        flag : flag,
                        pg_pool : pg,
                        redis_pool : redis,
                        id : redis_p.0,
                        real_worker_fn : worker
                    });

                    pool_args.push((args, &wrapper_worker_fn));
                }
                self.t_pool.use_pool_from_vec(pool_args);
            }
        }
    }
}


