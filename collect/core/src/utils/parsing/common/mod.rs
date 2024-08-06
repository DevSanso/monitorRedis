use std::error::Error;

use crate::macros::utils_new_error_crate;

#[inline]
pub fn split_eq_tuple(s : &'_ str) -> Result<(String, String), Box<dyn Error>> {
    let v : Vec<&str> = s.split("=").collect();
    if v.len() != 2 {
        return utils_new_error_crate!(data, CantSplitTupleError, s);
    }

    Ok((String::from(v[0]), String::from(v[1])))
}

#[inline]
pub fn split_rest_tuple<'a>(s : &'a str) -> Result<(&'a str, &'a str), Box<dyn Error>> {
    let v : Vec<&str> = s.split(",").collect();

    if v.len() != 2 {
        return utils_new_error_crate!(data, CantSplitTupleError, s);
    }

    Ok((v[0], v[1]))
}


#[inline]
pub fn split_colon_tuple(s : &'_ str) -> Result<(String, String), Box<dyn Error>> {
    let v : Vec<&str> = s.split(":").collect();
    if v.len() != 2 {
        return utils_new_error_crate!(data, CantSplitTupleError, s);
    }

    Ok((String::from(v[0]), String::from(v[1])))
}

pub fn split_line_and_fold_data<T, F, F2>(line : &'_ str, tuple_gen : F, mapping_val_fn : F2) -> Result<T, Box<dyn Error>> 
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