use std::sync::{Arc,Mutex};
use std::error::Error;

use dbs::pg_pool::PgPool;
use dbs::redis_pool::RedisPool;

pub type WorkerFn = &'static (dyn Fn(i32, &'_ mut dbs::redis_pool::RedisRequester, &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> + Send + Sync);
pub struct WrapperWorkerArgs {
    pub flag : Arc<Mutex<bool>>,
    pub pg_pool : Arc<Mutex<PgPool>>,
    pub redis_pool : Arc<Mutex<RedisPool>>,
    pub id : i32,
    pub real_worker_fn : WorkerFn
}

pub type WrapperWokerFn = &'static (dyn Fn(Option<WrapperWorkerArgs>) -> Result<(), Box<dyn Error>> + Send + Sync);