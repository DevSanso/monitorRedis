use std::error::Error;
use std::num::ParseIntError;

use core::utils::parsing::common::*;
use core::utils_new_error;
use core::utils_inherit_error;

#[derive(Default, Clone)]
pub struct KeyMemUsage {
    pub name : String,
    pub mem_size : i64,
    pub remain_expired_time : i64
}

#[derive(Default)]
pub struct KeyMemUsageList {
    pub next_cursor : i64,
    pub fetch : Vec<KeyMemUsage>
}

pub fn parsing_key_usage_top_ten_hundred(res : String) -> Result<KeyMemUsageList, Box<dyn Error>> {
    let mut token = res.split("\n");
    
    let cursor = match token.next() {
        Some(s) => s,
        None => return utils_new_error!(fetch, NilDataError, "key usage top, no cursor data")
    };

    let cursor_cast :i64 = match cursor.parse::<i64>() {
        Ok(ok) => ok,
        Err(err) => return utils_inherit_error!(data, GetDataCastError, "", err)
    };

    let mut ret = KeyMemUsageList::default();
    ret.next_cursor = cursor_cast;

    loop {
        let line = token.next();
        if line.is_none() {
            break;
        }

        let name = String::from(line.unwrap());
        if name == "" {
            continue;
        }

        let key_size = match token.next() {
            Some(val) => match val.parse::<i64>() {
                Ok(ok) => ok,
                Err(err) => return utils_inherit_error!(data, GetDataCastError, "", err)
            },
            None => return utils_new_error!(proc,NoneDataError, "key_usage, key_size data is none")
        };

        let remain_expired = match token.next() {
            Some(val) => match val.parse::<i64>() {
                Ok(ok) => ok,
                Err(err) => return utils_inherit_error!(data, GetDataCastError, "", err)
            },
            None => return utils_new_error!(proc,NoneDataError, "key_usage, remain_expired data is none")
        };

        ret.fetch.push(KeyMemUsage { name: name, mem_size: key_size, remain_expired_time: remain_expired });
    }
    
    Ok(ret)
}