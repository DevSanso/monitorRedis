use std::error::Error;

use crate::errs::ReponseParsingError;

pub fn parsing_confg_get_databases(res : String) -> Result<i64, Box<dyn Error>> {
    let v : Vec<&str> = res.split("\n").collect();
    if v.len() < 2 {
        return Err(Box::new(ReponseParsingError));
    }

    if v[0] != "databases" {
        return Err(Box::new(ReponseParsingError));
    }
    let parse = v[1].parse::<i64>();
    if parse.is_err() {
        return Err(Box::new(ReponseParsingError));
    }

    Ok(parse.unwrap())
}

pub fn parsing_dbsize(res : String) -> Result<i64, Box<dyn Error>> {
    let v : Vec<&str> = res.split_whitespace().collect();
    if v.len() < 2 {
        return Err(Box::new(ReponseParsingError));
    }

    if v[0] != "(integer)" {
        return Err(Box::new(ReponseParsingError));
    }
    let parse = v[1].parse::<i64>();
    if parse.is_err() {
        return Err(Box::new(ReponseParsingError));
    }

    Ok(parse.unwrap())
}