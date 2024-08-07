use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use rusqlite::{Connection, OpenFlags, Rows, Row};
use rusqlite::types::{ToSql, ToSqlOutput, Type, Value, ValueRef};

use log::*;

use core::structure::owned_pool::OwnedPool;

use core::utils_inherit_error;
use core::utils_new_error;

pub type SqlitePoolAlias = Arc<OwnedPool<SqliteConn, ()>>;


pub fn new_sqlite_pool(name : String, path : String, max_size : usize) -> SqlitePoolAlias {
    OwnedPool::new(name, Box::new( move |_ : () | -> Option<SqliteConn> {
        let p = path.clone();
        let c = match p.as_str() {
            "" => Connection::open_in_memory_with_flags(OpenFlags::SQLITE_OPEN_READ_WRITE),
            _ => Connection::open_with_flags(p.as_str(), OpenFlags::SQLITE_OPEN_READ_ONLY)
        };

        if c.is_err() {
            trace!("SqlitePool - gen : {}", c.err().unwrap());
            return None;
        }

        Some(SqliteConn::new(c.unwrap()))
    }), max_size)
}

#[derive(Default)]
pub struct SqliteRows {
    cache : HashMap<(usize, usize),Option<Box<dyn ToSql>>>,
    col_data_type : HashMap<usize, Type>,
    col_size : usize,
    row_size : usize,
    syntax : &'static str
}
impl SqliteRows {
    pub(super) fn new<'a>(mut rs : Rows<'a>, cols_size : usize, syntax : &'static str) -> Result<SqliteRows, Box<dyn Error>> {
        let mut ret = Self::default();
        let mut row_idx : usize = 0;
        while let Some(s) = {
            let r = rs.next();
            if r.is_err() {
                return utils_inherit_error!(connection, ResponseScanError, "", r.err().unwrap());
            }
            r.unwrap()
        } {
            if row_idx == 0 {Self::register_data_type(&mut ret, cols_size, s)?;}

            Self::register_data(&mut ret, row_idx, cols_size,  s)?;
            row_idx += 1;
        }

        ret.col_size = cols_size;
        ret.row_size = row_idx;         
        ret.syntax = syntax;
        Ok(ret)
    }

    pub fn row_len(&self) -> usize {self.row_size}

    fn register_data_type<'a>(refer : &mut Self, col_size : usize, r :&'a Row<'a>) -> Result<(), Box<dyn Error>> {
        for idx in 0..col_size {
            let data = r.get_ref(idx)?;
            let data_type = data.data_type();
            refer.col_data_type.insert(idx, data_type);
           
        }
        Ok(())
    }

    fn register_data<'a>(refer : &mut Self, row_idx : usize, col_size : usize, r :&'a Row<'a>) -> Result<(), Box<dyn Error>> {
        for idx in 0..col_size {
            let data = r.get_ref(idx)?;
            let data_type = data.data_type();
            let left_data_type = &refer.col_data_type[&idx];

            if *left_data_type != data_type {
                return utils_new_error!(data, GetDataCastError, format!("{}:{}", left_data_type.to_string(), data_type.to_string()));
            }

            let raw_data = Self::get_tosql(&data, data_type)?;
            refer.cache.insert((row_idx, idx), raw_data);
        }
        Ok(())
    }

    fn get_tosql<'a>(vref : &'a ValueRef, t : Type) ->Result<Option<Box<dyn ToSql>>, Box<dyn Error>>{
        let s : Option<Box<dyn ToSql>> = match t {
            Type::Null => None,
            Type::Blob => Some(Box::new(Vec::from(vref.as_bytes()?))),
            Type::Integer => Some(Box::new(vref.as_i64()?.clone())),
            Type::Real => Some(Box::new(vref.as_f64()?.clone())),
            Type::Text => Some(Box::new(String::from(vref.as_str()?)))
        };

        Ok(s)
    }

   pub fn get_i64_data(&self, row_idx : usize, col_idx : usize) -> Result<Option<i64>, Box<dyn Error>> {
        let t = &self.col_data_type[&col_idx];
        if *t != Type::Integer && *t != Type::Null {
            return utils_new_error!(data, GetDataCastError, format!("{}[{}:{}]", self.syntax, row_idx, col_idx));
        }
        
        let data = match self.cache.get(&(row_idx, col_idx)) {
            Some(s) => s,
            None => return utils_new_error!(proc, CriticalError, format!("get data is None, critical code error [{}:{}],", row_idx, col_idx))
        };

        if *t == Type::Null {
            return Ok(None);
        }

        let to_sql = data.as_ref().unwrap();
        let output = match to_sql.to_sql().unwrap() {
            ToSqlOutput::Owned(val) => val,
            _ => return utils_new_error!(proc, CriticalError, format!("get sql data enum not matching, check code [{}:{}],", row_idx, col_idx))
        };

        let ret : Option<i64> = match output {
            Value::Integer(i) => Some(i),
            Value::Null => None,
            _ => return utils_new_error!(proc, CriticalError, format!("col data is not integer, check code [{}:{}],", row_idx, col_idx))
        };

        Ok(ret)
    }

    pub fn get_f64_data(&self, row_idx : usize, col_idx : usize) -> Result<Option<f64>, Box<dyn Error>> {
        let t = &self.col_data_type[&col_idx];
        if *t != Type::Real && *t != Type::Null {
            return utils_new_error!(data, GetDataCastError, format!("{}[{}:{}]", self.syntax, row_idx, col_idx));
        }
        
        let data = match self.cache.get(&(row_idx, col_idx)) {
            Some(s) => s,
            None => return utils_new_error!(proc, CriticalError, format!("get data is None, critical code error [{}:{}],", row_idx, col_idx))
        };

        if *t == Type::Null {
            return Ok(None);
        }

        let to_sql = data.as_ref().unwrap();
        let output = match to_sql.to_sql().unwrap() {
            ToSqlOutput::Owned(val) => val,
            _ => return utils_new_error!(proc, CriticalError, format!("get sql data enum not matching, check code [{}:{}],", row_idx, col_idx))
        };

        let ret : Option<f64> = match output {
            Value::Real(i) => Some(i),
            Value::Null => None,
            _ =>  return utils_new_error!(proc, CriticalError, format!("col data is not float, check code [{}:{}],", row_idx, col_idx))
        };

        Ok(ret)
    }

    pub fn get_str_data(&self, row_idx : usize, col_idx : usize) -> Result<Option<String>, Box<dyn Error>> {
        let t = &self.col_data_type[&col_idx];
        if *t != Type::Text && *t != Type::Null {
            return utils_new_error!(data, GetDataCastError, format!("{}[{}:{}]", self.syntax, row_idx, col_idx));
        }
        
        let data = match self.cache.get(&(row_idx, col_idx)) {
            Some(s) => s,
            None => return utils_new_error!(proc, CriticalError, format!("get data is None, critical code error [{}:{}],", row_idx, col_idx))
        };

        if *t == Type::Null {
            return Ok(None);
        }

        let to_sql = data.as_ref().unwrap();
        let output = match to_sql.to_sql().unwrap() {
            ToSqlOutput::Borrowed(r) => r,
            _ => return utils_new_error!(proc, CriticalError, format!("get sql data enum not matching, check code [{}:{}],", row_idx, col_idx))
        };

        let ret : Option<String> = match {
            let d = output.as_str_or_null();
            if d.is_err() {
                return utils_new_error!(proc, CriticalError, format!("col data is not string, check code [{}:{}],", row_idx, col_idx))
            }
            d.unwrap()
        } {
            Some(s) => Some(String::from(s)),
            None => None
        };

        Ok(ret)
    }

}

pub struct SqliteConn {
    conn : Connection
}
impl SqliteConn {
    pub(super) fn new(raw : Connection) -> Self {
        SqliteConn {
            conn : raw
        }
    }

    pub fn query<F : 'static>(&mut self, 
        query : String, args : &'_[&'_ dyn ToSql], gen : impl Fn(SqliteRows) -> Result<F, Box<dyn Error>>, syntax : &'static str) -> Result<F, Box<dyn Error>>{
        let mut stmt = self.conn.prepare(query.as_str())?;
        let data_key_cnt = stmt.column_count();
        
        let rows = stmt.query(args)?;

        let srs = SqliteRows::new(rows, data_key_cnt, syntax)?;
        gen(srs)
    }

    pub fn execute(&mut self, query : String,  args : &'_[&'_ dyn ToSql]) -> Result<usize, Box<dyn Error>> {
        let ret =self.conn.execute(query.as_str(),args)?;
        Ok(ret)
    }

}

#[cfg(test)]
mod pool_tests {

    #[test]
    fn pool_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut p = super::new_sqlite_pool(String::from("test"),String::from(""), 10);

        {
            let mut conn = p.get_owned(())?;
            let real_conn = conn.get_value();
            real_conn.execute("create table a (b int, f real, c varchar(12), d varchar(12));".to_string() , &[])?;
            real_conn.execute("insert into a values(?,1.0,?,NULL)".to_string(), &[&(12 as i64), &"123".to_string()])?;
            real_conn.execute("insert into a values(?,1.0,?,NULL)".to_string(), &[&(12 as i64), &"123".to_string()])?;
        }


        {
            let mut conn = p.get_owned(())?;
            let real_conn = conn.get_value();
            real_conn.query("select * from a".to_string() , &[],|x| {
                assert_eq!(x.get_i64_data(0, 0)?, x.get_i64_data(1, 0)?,"1 row not eq");
                assert_eq!(x.get_f64_data(0, 1)?, x.get_f64_data(1, 1)?,"2 row not eq");
                assert_eq!(x.get_str_data(0, 2)?, x.get_str_data(1, 2)?,"3 row not eq");
                assert_eq!(x.get_str_data(0, 3)?, x.get_str_data(1, 3)?,"4 row not eq");
                assert_eq!(x.row_len(), 2);
                Ok(())

               
            }, "test")?;
        }

        Ok(())
    }
}