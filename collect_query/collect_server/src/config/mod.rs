use std::error::Error;

use serde::{Deserialize};

use serde_json;

use crate::typed::DbConnConfig;
use core::utils_inherit_error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pg_config : DbConnConfig<String>,
    pub sqlite_path : String
}

impl Config {
    pub fn new_from_json(data : &'_ str) -> Result<Self,Box<dyn Error>> {
        serde_json::from_str(data).or_else(|x| {
            utils_inherit_error!(data, GetDataCastError, "config cast error", x)?
        })
    }
}