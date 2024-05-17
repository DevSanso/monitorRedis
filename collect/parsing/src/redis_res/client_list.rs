use std::error::Error;

use crate::redis_res::split_eq_tuple;
use crate::redis_res::split_line_and_fold_data;
use crate::redis_res::CantMappingValueError;

#[derive(Default)]
pub struct ClientListItem {
    pub id : u64,
    pub addr : String,
    pub fd : u64,
    pub name :String,
    pub age : u64,
    pub idle : u64,
    pub flags : char,
    pub db : u64,
    pub sub : u64, //channel
    pub psub : u64, //parttern
    pub multi : i32,
    pub qbuf : u64,
    pub qbuf_free : u64,
    pub obl : u64,
    pub oll : u64,
    pub omem : u64,
    pub events : char,
    pub cmd : String
}

pub type ClientList = Vec<ClientListItem>;

#[inline]
fn mapping_client_list_item(key : &'_ str, value : &'_ str, refer : &mut ClientListItem) -> Result<(), Box<dyn Error>> {
    match key {
        "id" => refer.id = value.parse()?,
        "addr" => refer.addr = String::from(value),
        "fd" => refer.fd = value.parse()?,
        "name" => refer.name = String::from(value),
        "age" => refer.age = value.parse()?,
        "idle" => refer.idle = value.parse()?,
        "flags" => refer.flags = value.parse()?,
        "db" => refer.db = value.parse()?,
        "sub" => refer.sub = value.parse()?,
        "psub" => refer.psub = value.parse()?,
        "multi" => refer.multi = value.parse()?,
        "qbuf" => refer.qbuf = value.parse()?,
        "qbuf-free" => refer.qbuf_free = value.parse()?,
        "obl" => refer.obl = value.parse()?,
        "oll" => refer.oll = value.parse()?,
        "omem" => refer.omem = value.parse()?,
        "events" => refer.events = value.chars().next().unwrap(),
        "cmd" => refer.cmd = String::from(value),
        _ => return Err(Box::new(CantMappingValueError::new(String::from(key))))
    }

    Ok(())
}

pub fn parsing_client_list(res : String) -> Result<ClientList, Box<dyn Error>> {
    let s = res.as_str();
    let mut list = Vec::<ClientListItem>::new();

    for line in s.split("\n") {
        let obj = split_line_and_fold_data(line, split_eq_tuple, mapping_client_list_item)?;
        list.push(obj);
    }

    Ok(list)
}

#[cfg(test)]
mod client_list_tests {
    use std::error::Error;
    #[test]
    pub fn test_parsing_client_list() -> Result<(), Box<dyn Error>> {
        let test_data = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/test/data/redis/client_list.txt"));
        super::parsing_client_list(String::from(test_data))?;

        Ok(())
    }
}