mod config;
mod worker;
mod threads;
mod typed;
mod db_info;
mod utils;
mod collector;
mod global;
mod args;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::error::Error;
use core::utils_new_error;


use logger::{init_logger, LoggerConfig};
use args::Args;

fn main() -> Result<(), Box<dyn Error>>{
    #[cfg(not(feature = "runTest"))]
    let args_opt = Args::new();

    if args_opt.is_none() {
        return utils_new_error!(proc, NoneDataError, "args parsing failed");
    }

    let args = args_opt.unwrap();

    let log_cfgs = vec![LoggerConfig::new(args.log_level.clone(), if args.log_file_path == "" {
        Some(args.log_file_path.clone())
    }else {
        None
    })];

    init_logger(log_cfgs)?;

    global::init_global(args)?;

    let shutdown = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&shutdown))?;

    loop {
        if !shutdown.load(Ordering::Relaxed) {

            break;
        }
    }

    Ok(())
}
