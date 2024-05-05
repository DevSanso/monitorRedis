pub struct DbConnConfig<T> {
    pub user : String,
    pub password : String,
    pub db_name : T,
    pub ip : String,
    pub port : u32
}

pub struct Config {
    pub pg_config : DbConnConfig<String>,
    pub target_configs : Vec<DbConnConfig<u32>>,

    pub logger_path : String,
    pub logger_level : String
}