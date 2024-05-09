use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct DbConnConfig<T> {
    pub user : String,
    pub password : String,
    pub db_name : T,
    pub ip : String,
    pub port : u32
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pg_config : DbConnConfig<String>,
    pub sqlite_path : String,

    pub logger_path : Option<String>,
    pub logger_level : String
}