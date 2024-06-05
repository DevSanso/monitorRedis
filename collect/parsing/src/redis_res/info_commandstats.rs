use std::{error::Error, str::Split};

use super::split_colon_tuple;
use crate::errs::CantMappingValueError;

#[derive(Default, Debug)]
pub struct CommandStat {
    pub cmd : String,
    pub calls : i64,
    pub usec : i64,
    pub usec_per_call : f64 
}
pub fn parsing_commandstat_datas(cmd : String, datas : Split<&'_ str>) -> Result<CommandStat, Box<dyn Error>> {
    let mut c = CommandStat::default();
    c.cmd = cmd.replace("cmdstat_", "");

    for data in datas {
        match data {
            "calls" => c.calls = data.parse()?,
            "usec" => c.usec = data.parse()?,
            "usec_per_call" => c.usec_per_call = data.parse()?,
            _ =>  return Err(Box::new(CantMappingValueError::new(String::from(data))))
        }
    }

    Ok(c)
}
pub fn parsing_info_commandstats(res : String) -> Result<Vec<CommandStat>, Box<dyn Error>> {
    let str_p = res.as_str();
    let split = str_p.split("\n").fold(Vec::<&'_ str>::new(), |mut acc, x | {
        acc.push(x.trim());

        acc
    });

    let mut v = Vec::new();
    
    for stri in split {
        let temp = split_colon_tuple(stri)?;
        let datas = temp.1.split(",");
        let obj = parsing_commandstat_datas(temp.0, datas)?;
        v.push(obj);
    }

    Ok(v)
}