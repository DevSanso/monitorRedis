use std::{error::Error, num::{ParseFloatError, ParseIntError}, str::Split};

use core::utils::parsing::common::*;

use core::utils_new_error;
use core::utils_inherit_error;

#[derive(Default, Debug)]
pub struct CommandStat {
    pub cmd : String,
    pub calls : i64,
    pub usec : i64,
    pub usec_per_call : f64 
}
pub fn parsing_commandstat_datas(cmd : String, datas : Split<&'_ str>) -> Result<CommandStat, Box<dyn Error>> {
    const UNSUPPORT_KEYS : &'static [&'static str]= &["rejected_calls", "failed_calls"];
    let mut c = CommandStat::default();
    c.cmd = cmd.replace("cmdstat_", "");

    for data in datas {
        let (key, val) = split_eq_tuple(data)?;
        match key.as_str() {
            "calls" => c.calls = val.parse()?,
            "usec" => c.usec = val.parse()?,
            "usec_per_call" => c.usec_per_call = val.parse()?,
            k if UNSUPPORT_KEYS.contains(&k) => {
                log::debug!("not support {} this commandstat data", k);
            },            
            _ =>  return utils_new_error!(data, CantMappingKeyError, data)
        }
    }

    Ok(c)
}
pub fn parsing_info_commandstats(res : String) -> Result<Vec<CommandStat>, Box<dyn Error>> {
    let str_p = res.as_str();
    let split = str_p.trim().split("\n").skip(1).fold(Vec::<&'_ str>::new(), |mut acc, x | {
        acc.push(x);
        acc
    });

    let mut v = Vec::new();
    
    for stri in split {
        let temp = split_colon_tuple(stri)?;
        let datas = temp.1.split(",");
        let obj = parsing_commandstat_datas(temp.0, datas);

        if obj.is_err() {
            let err = obj.err().unwrap();
            if err.is::<ParseFloatError>() || err.is::<ParseIntError>() {
                return utils_inherit_error!(data, GetDataCastError, "", err);
            }
            else {
                return Err(err);
            }
        }
        else {
            v.push(obj.unwrap());
        }
    }

    Ok(v)
}