mod config;
mod typed;
mod db_info;
mod utils;
mod collector;
mod global;
mod args;
mod executor;
mod interval;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::error::Error;
use core::utils_new_error;
use log::info;

use logger::{init_logger, LoggerConfig};
use args::Args;

fn main() -> Result<(), Box<dyn Error>>{
    let args_opt = Args::new();

    if args_opt.is_none() {
        return utils_new_error!(proc, NoneDataError, "args parsing failed");
    }

    let args = args_opt.unwrap();

    let log_cfgs = vec![LoggerConfig::new(args.log_level.clone(), if args.log_file_path != "" {
        Some(args.log_file_path.clone())
    }else {
        None
    })];

    init_logger(log_cfgs)?;

    let server_type = args.server_type.clone();
    let server_id = args.server_id.clone();

    global::init_global(args)?;

    let execute = executor::ThreadExecutor::new();
    execute.run_and_nonblocking(server_type.clone());

    let shutdown = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&shutdown))?;

    loop {
        if !shutdown.load(Ordering::Relaxed) {
            info!("shutdown collect --server-type {} --server_id {}", server_type, server_id);
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }

    Ok(())
}
