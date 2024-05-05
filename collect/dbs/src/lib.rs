use std::error::Error;
use std::fmt::Display;

use core::impl_error_structure;


pub mod redis_pool;
pub mod pg_pool;

impl_error_structure!(NotMatchArgsLenError, "query bound args count not mathcing");