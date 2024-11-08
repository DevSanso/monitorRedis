use std::error::Error;

use core::utils::parsing::common::*;
use core::utils_new_error;
use core::utils_inherit_error;
/*
# CPU
used_cpu_sys:1.944891
used_cpu_user:1.398170
used_cpu_sys_children:0.000000
used_cpu_user_children:0.000000


*/
#[derive(Default,Debug)]
pub struct InfoCpu {
    pub cpu_sys : f64,
    pub cpu_user : f64,
    pub child_cpu_sys : f64,
    pub child_cpu_user : f64,
    pub total_sec : i64
}

#[inline]
fn mapping_info_cpu(r : &mut InfoCpu, raw_data : &'_ str) -> Result<(), Box<dyn Error>> {
    const UNSUPPORT_KEYS : &'static [&'static str]= &["used_cpu_sys_main_thread", "used_cpu_user_main_thread"];

    let s = split_colon_tuple(raw_data)?;

    match s.0 {
        "used_cpu_sys" => r.cpu_sys = s.1.trim().parse()?,
        "used_cpu_user" => r.cpu_user = s.1.trim().parse()?,
        "used_cpu_sys_children" => r.child_cpu_sys = s.1.trim().parse()?,
        "used_cpu_user_children" => r.child_cpu_user = s.1.trim().parse()?,
        key if UNSUPPORT_KEYS.contains(&key) => {
            log::debug!("not support {} this client list data", key);
        },

        _ => return utils_new_error!(data, CantMappingKeyError, s.0)
    }

    Ok(())
}

pub fn parsing_info_cpu(res : String) -> Result<InfoCpu, Box<dyn Error>> {
    let s = res.as_str();
    let mut o = InfoCpu::default();

    let mut token = s.split("\n");

    let total = token.next().unwrap();

    o.total_sec = match total.parse() {
        Ok(ok) => ok,
        Err(err) => return utils_inherit_error!(data, CantMappingKeyError, "info cpu", err)
    };

    for line in token.skip(1) {
        if line == "" {continue;}
        let ret = mapping_info_cpu(&mut o, line);

        if ret.is_err() {
            let err = ret.err().unwrap();
            if !err.is::<core::errs::data::CantMappingKeyError>() {
                return utils_inherit_error!(data, GetDataCastError, "", err);
            }

            return Err(err);
        }
    }

    Ok(o)
}