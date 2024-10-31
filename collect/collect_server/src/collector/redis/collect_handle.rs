use std::error::Error;
use std::collections::HashMap;

use core::utils_new_error;
use dbs_cmd::{RedisCommand, REIDS_COMMANDLINE_MAP, CollectCommand};
use dbs::redis_pool::{RedisPoolAlias, RedisRequester};

use crate::utils::parsing::redis_res::KeyMemUsage;


pub fn client_list_handle(val : &str) {

}