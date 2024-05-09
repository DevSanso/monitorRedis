use std::error::Error;
use std::fmt::Display;

use core::impl_error_structure;


pub mod redis_pool;
pub mod pg_pool;
pub mod sqlite_pool;
pub mod utils;

impl_error_structure!(NotMatchArgsLenError, "query bound args count not mathcing");
impl_error_structure!(OutIndexRowError, "Out Index Select Row Error");