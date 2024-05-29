use std::error::Error;
use std::fmt::Display;

use core::impl_error_structure;

impl_error_structure!(MoreArgsError, "process args is 2 (ex: collect_server ../config.json");
impl_error_structure!(WrapperNoneArgsError, "this wrapper args require Option::Some args");
impl_error_structure!(arg, PgPoolGetError, "Postgresql Pool get failed");
impl_error_structure!(arg, RedisPoolGetError, "Redis Pool get failed");
impl_error_structure!(arg, WorkerRetureError, "Worker return is error");