use crate::typed::DbConnConfig;

pub struct RedisConnCfg {
    pub link_id : i32,
    pub conn_cfg : DbConnConfig<u32>,
    pub hash_code : Vec<u8>
}