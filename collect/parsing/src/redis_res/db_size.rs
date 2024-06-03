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
    Ok(res.parse()?)
}

#[cfg(test)]
mod db_size_tests {
    use std::error::Error;
    #[test]
    pub fn test_parsing_config_get_dbnum() -> Result<(), Box<dyn Error>> {
        let select_db = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/test/data/redis/db_size/select_db_nums.txt"));
        let num =  super::parsing_confg_get_databases(String::from(select_db))?;
        assert_eq!(num, 16, "not match db num");
        Ok(())
    }
    #[test]
    pub fn test_parsing_db_size() -> Result<(), Box<dyn Error>> {
        let db_size = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/test/data/redis/db_size/db_size.txt"));
        let size = super::parsing_dbsize(String::from(db_size))?;
        assert_eq!(size, 0, "not match db size");
        Ok(())
    }
}