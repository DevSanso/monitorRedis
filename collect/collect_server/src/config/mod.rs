use serde::{Deserialize};

use crate::typed::DbConnConfig;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pg_config : DbConnConfig<String>,
    pub sqlite_path : String,

    pub logger_path : Option<String>,
    pub logger_level : String
}