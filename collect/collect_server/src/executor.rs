use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::ops::IndexMut;
use log::error;

use thread_pool::{TPool,TItem};
use dbs_cmd::RedisCommand;
use crate::interval::get_redis_current_interval;
use crate::collector::Collector;
use crate::collector::redis::{make_redis_collector, RedisCollector};
use core::utils_new_error;
use crate::global::get_redis_global;


type RedisFnArgs = (Box<RedisCollector>, Arc<Mutex<HashMap<RedisCommand, bool>>>);
type RedisFn = &'static (dyn Fn(Option<RedisFnArgs>) -> Result<(), Box<dyn std::error::Error>> + Send + Sync);
pub struct ThreadExecutor {
    run_flag : Arc<Mutex<HashMap<RedisCommand, bool>>>,
    pool : Mutex<TPool<RedisFnArgs>>
}

macro_rules! check_cmd_run {
    ($exec:expr, $c:expr) => {
        {
            let is_run = $exec.run_flag.lock().unwrap();
            let val = is_run.get($c);
            if val.is_some() || val.unwrap() == &true {
                continue;
            }
        }
    };
}

pub fn make_redis_thread_fn(run_f :&Arc<Mutex<HashMap<RedisCommand, bool>>>, server_id : i32, cmd : RedisCommand) -> (Option<RedisFnArgs>, RedisFn) {
    let c = make_redis_collector(server_id, cmd);
    let f_clone = Arc::clone(run_f);

    {
        let mut f = run_f.lock().unwrap();
        *f.entry(c.get_cmd()).or_insert(true) = true;
    }

    (Some((c, f_clone)), &|x| {
        {
            let args = x.unwrap();
            let mut collector = args.0;
            let flag = args.1;

            collector.run_collect();

            {
                let mut f = flag.lock().unwrap();
                *f.get_mut(&collector.get_cmd()).unwrap() = false;
            }
        }
        Ok(())
    })
}

impl ThreadExecutor {
    pub fn new() -> Arc<Self> {
        let o = ThreadExecutor {
            run_flag : Arc::new(Mutex::new(HashMap::new())),
            pool : Mutex::new(TPool::new("collect_subthread", 10))
        };

        Arc::new(o)
    }

    fn redis_run_and_blocking(exec : Arc<Self>) {
        loop {
            let will_run_cmd = get_redis_current_interval();

            for cmd in will_run_cmd {
                check_cmd_run!(exec, &cmd);

                let run_data = make_redis_thread_fn(&exec.run_flag, get_redis_global().args.server_id, cmd);

                exec.pool.lock().unwrap().use_pool(run_data);
            }

            let millie = (chrono::Local::now().timestamp_millis() % 1000) as u64;
            thread::sleep(Duration::from_millis(1010 - millie));
        }
    }

    pub fn run_and_nonblocking(self : &Arc<Self>, server_type : String) {
        let clone = Arc::clone(&self);

        thread::spawn(move || {
            if server_type == "redis" {
                ThreadExecutor::redis_run_and_blocking(clone);
            }
            else {
                let err_msg: Result<(), Box<core::errs::proc::CriticalError>> = utils_new_error!(proc, CriticalError, format!("{} is not support", server_type));
                error!("{}", err_msg.unwrap_err());
            }
        });
    }
}