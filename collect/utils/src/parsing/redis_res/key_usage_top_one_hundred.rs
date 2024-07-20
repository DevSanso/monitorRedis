use std::error::Error;
use std::num::ParseIntError;

use crate::parsing::common::*;
use core::utils_new_error;
use core::utils_inherit_error;

#[derive(Default, Clone)]
pub struct KeyMemUsage {
    pub name : String,
    pub mem_size : i64
}

#[derive(Default)]
pub struct KeyMemUsageList {
    pub next_cursor : i64,
    pub fetch : Vec<KeyMemUsage>
}

pub fn parsing_key_usage_top_one_hundred(res : String) -> Result<KeyMemUsageList, Box<dyn Error>> {
    let mut token = res.split("\n");
    
    let cursor = match token.next() {
        Some(s) => s,
        None => return utils_new_error!(fetch, NilDataError, "key usage top, no cursor data")
    };

    let cursor_cast :i64 = match cursor.parse::<i64>() {
        Ok(ok) => ok,
        Err(err) => return utils_inherit_error!(data, GetDataCastError, "", err)
    };

    let mut temp_name : Option<String> = None;
    let mut ret = KeyMemUsageList::default();
    ret.next_cursor = cursor_cast;


    for line in token {
        if temp_name.is_some() {
            let key_size = match line.parse::<i64>() {
                Ok(ok) => ok,
                Err(err) => return utils_inherit_error!(data, GetDataCastError, "", err)
            };

            ret.fetch.push(KeyMemUsage { name: temp_name.take().unwrap(), mem_size: key_size });
            continue;
        }

        temp_name = Some(String::from(line));
    }
    
    Ok(ret)
}