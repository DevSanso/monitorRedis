

pub fn create_pg_url(username : &'_ str, password : &'_ str, addr : &'_ str, port : u32, db_name : &'_ str) -> String {
    format!("postgresql://{username}:{password}@{addr}:{port}/{db_name}?connect_timeout=60")
}

pub fn create_redis_url(username : &'_ str, password : &'_ str, addr : &'_ str, port : u32, db_name : u32) -> String {
    format!("redis://{username}:{password}@{addr}:{port}/{db_name}?connect_timeout=60")
}

use rust_decimal::{prelude::FromPrimitive, Decimal};
pub fn make_pg_numeric(value : f64) -> Decimal {
    Decimal::from_f64(value).unwrap()
}