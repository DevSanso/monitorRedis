use std::error::Error;

use super::split_colon_tuple;
use crate::errs::CantMappingValueError;
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
    pub child_cpu_user : f64
}

#[inline]
fn mapping_info_cpu(r : &mut InfoCpu, raw_data : &'_ str) -> Result<(), Box<dyn Error>> {
    let s = split_colon_tuple(raw_data)?;

    match s.0.as_str() {
        "used_cpu_sys" => r.cpu_sys = s.1.as_str().trim().parse()?,
        "used_cpu_user" => r.cpu_user = s.1.as_str().trim().parse()?,
        "used_cpu_sys_children" => r.child_cpu_sys = s.1.as_str().trim().parse()?,
        "used_cpu_user_children" => r.child_cpu_user = s.1.as_str().trim().parse()?,
        _ => return Err(Box::new(CantMappingValueError::new(String::from(s.0.as_str()))))
    }

    Ok(())
}

pub fn parsing_info_cpu(res : String) -> Result<InfoCpu, Box<dyn Error>> {
    let s = res.as_str();
    let mut o = InfoCpu::default();

    for line in s.split("\n").skip(1) {
        if line == "" {continue;}
        mapping_info_cpu(&mut o, line)?;

    }

    Ok(o)
}