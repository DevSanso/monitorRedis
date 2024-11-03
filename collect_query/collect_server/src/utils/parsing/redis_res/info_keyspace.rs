use std::error::Error;

use core::utils::parsing::common::*;

/**
 * # Keyspace
* db0:keys=795700,expires=0,avg_ttl=0
*/
pub struct InfoKeySpace {
    pub db_name : String,
    pub key_cnt : i64,
    pub expire_cnt : i64,
    pub avg_ttl : i64
}

pub type InfoKeySpaceList = Vec<InfoKeySpace>;

fn mapping_info_keyspace(line :&'_ str) -> Result<InfoKeySpace, Box<dyn Error>> {
    let tuple = split_colon_tuple(line)?;
    let datas : Vec<&str> = tuple.1.split(",").collect();
    
    Ok(InfoKeySpace {
        db_name : tuple.0.to_string(),
        key_cnt : {
            split_eq_tuple(datas[0])?.1.parse()?
        },
        expire_cnt : {
            split_eq_tuple(datas[1])?.1.parse()?
        }
        ,
        avg_ttl : {
            split_eq_tuple(datas[2])?.1.parse()?
        }
    })

}

pub fn parsing_info_keyspace(res : String) -> Result<InfoKeySpaceList, Box<dyn Error>> {
    let lines = res.split("\n").skip(1).filter(|x| *x != "");

    let mut v = Vec::new();

    for line in lines {
        let item = mapping_info_keyspace(line.trim())?;
        v.push(item);
    }

    Ok(v)
}
