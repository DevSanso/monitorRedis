use crate::typed::DbConnConfig;

pub struct RedisConnCfg {
    pub server_id : i32,
    pub conn_cfg : DbConnConfig<u32>,
    pub hash_code : Vec<u8>
}