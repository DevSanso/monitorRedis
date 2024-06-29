use std::error::Error;
use std::num::ParseIntError;

use crate::parsing::common::*;
use core::utils_new_error;
use core::utils_inherit_error;

#[derive(Default)]
pub struct ClientListItem {
    pub id : i64,
    pub addr : String,
    pub fd : i64,
    pub name :String,
    pub age : i64,
    pub idle : i64,
    pub flags : char,
    pub db : i64,
    pub sub : i64, //channel
    pub psub : i64, //parttern
    pub multi : i32,
    pub qbuf : i64,
    pub qbuf_free : i64,
    pub obl : i64,
    pub oll : i64,
    pub omem : i64,
    pub events : char,
    pub cmd : String,
    pub user : String
}

pub type ClientList = Vec<ClientListItem>;

#[inline]
fn mapping_client_list_item(key : &'_ str, value : &'_ str, refer : &mut ClientListItem) -> Result<(), Box<dyn Error>> {
    const UNSUPPORT_KEYS : &'static [&'static str]= &["laddr", "ssub", "argv-mem", "multi-mem", "rbs", "rbp", "tot-mem", "redir", "resp", "lib-name", "lib-ver"];

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
        "user" => refer.user = String::from(value),
        key if UNSUPPORT_KEYS.contains(&key) => {
            log::debug!("not support {} this client list data", key);
        }
        
        _ => return utils_new_error!(data, CantMappingKeyError, key)
    }

    Ok(())
}

pub fn parsing_client_list(res : String) -> Result<ClientList, Box<dyn Error>> {
    let s = res.as_str();
    let mut list = Vec::<ClientListItem>::new();

    for line in s.split("\n") {
        if line == "" {continue;}
        let obj = split_line_and_fold_data(line, split_eq_tuple, mapping_client_list_item);
        if obj.is_err() {
            let err = obj.err().unwrap();
            
            if err.is::<ParseIntError>() {
                return utils_inherit_error!(data, GetDataCastError, "", err);
            }
            else {
                return Err(err);
            }
        }else {
            list.push(obj.unwrap());
        }
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