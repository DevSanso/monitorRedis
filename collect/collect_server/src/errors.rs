use std::error::Error;
use std::fmt::Display;

use core::impl_error_structure;

impl_error_structure!(MoreArgsError, "process args is 2 (ex: collect_server ../config.json");