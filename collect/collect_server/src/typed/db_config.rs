use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct DbConnConfig<T> {
    pub user : String,
    pub password : String,
    pub db_name : T,
    pub ip : String,
    pub port : u32
}