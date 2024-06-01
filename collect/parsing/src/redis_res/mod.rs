mod client_list;
mod info_cpu;
mod info_stat;
mod db_size;

use std::error::Error;
use crate::errs::CantCastTupleError;

#[inline]
pub(crate) fn split_eq_tuple(s : &'_ str) -> Result<(String, String), Box<dyn Error>> {
    let v : Vec<&str> = s.split("=").collect();
    if v.len() != 2 {
        return Err(Box::new(CantCastTupleError));
    }

    Ok((String::from(v[0]), String::from(v[1])))
}

#[inline]
pub(crate) fn split_colon_tuple(s : &'_ str) -> Result<(String, String), Box<dyn Error>> {
    let v : Vec<&str> = s.split(":").collect();
    if v.len() != 2 {
        return Err(Box::new(CantCastTupleError));
    }

    Ok((String::from(v[0]), String::from(v[1])))
}

pub(crate) fn split_line_and_fold_data<T, F, F2>(line : &'_ str, tuple_gen : F, mapping_val_fn : F2) -> Result<T,Box<dyn Error>> 
where T : Default, F : Fn(&'_ str)-> Result<(String, String), Box<dyn Error>>,
F2 : Fn(&'_ str, &'_ str, &mut T) -> Result<(), Box<dyn Error>>{
    let mut ret = T::default();

    let v : Vec<&str> = line.split(" ").collect();
  
    for item in v {
        
        
        let (key,value) = tuple_gen(item)?;
        mapping_val_fn(key.as_str(), value.as_str(), &mut ret)?;
    }

    Ok(ret)
}   

pub use client_list::parsing_client_list;
pub use info_cpu::parsing_info_cpu;
pub use info_stat::parsing_info_stat;

pub use db_size::parsing_confg_get_databases;
pub use db_size::parsing_dbsize;