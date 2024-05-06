

pub fn create_pg_url(username : &'_ str, password : &'_ str, addr : &'_ str, port : u32, db_name : &'_ str) -> String {
    format!("postgresql://{username}:{password}@{addr}:{port}/{db_name}?connect_timeout=60")
}
