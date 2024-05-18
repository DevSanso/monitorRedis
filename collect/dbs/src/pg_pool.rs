use std::error::Error;

use postgres;

use core::structure::pool::{Pool, PoolItem};
use crate::errs::OutIndexRowError;

pub struct PgPool {
    pool : Pool<PgUploader, String>,
    url : String
}

impl PgPool {
    pub fn new(url : String) -> Self {
        PgPool {
            pool : Pool::new(Box::new(PgPool::gen), 10),
            url
        }
    }

    fn gen(url : String) -> Option<PgUploader> {
        match postgres::Client::connect(url.as_str(), postgres::NoTls) {
            Ok(client) => Some(PgUploader::new(client)),
            Err(_) => None
        }
    }

    pub fn get(&mut self) -> Result<PoolItem<PgUploader>, Box<dyn Error>> {
        self.pool.get(self.url.clone())
    }
}

pub struct PgRows {
    pub(crate) data : Vec<postgres::Row>
}

impl PgRows {
    pub fn len(&mut self) -> usize {self.len()}

    pub fn get_data_u32(&mut self, row_index : usize, col_index : usize) -> Result<u32, Box<dyn Error>> {
        if row_index >= self.data.len() {
            return Err(Box::new(OutIndexRowError));
        }

        let ret : u32 = self.data[row_index].try_get(col_index)?;
        Ok(ret)
    }

    pub fn get_data_f64(&mut self, row_index : usize, col_index : usize) -> Result<f64, Box<dyn Error>> {
        if row_index >= self.data.len() {
            return Err(Box::new(OutIndexRowError));
        }

        let ret : f64 = self.data[row_index].try_get(col_index)?;
        Ok(ret)
    }

    pub fn get_data_string(&mut self, row_index : usize, col_index : usize) -> Result<String, Box<dyn Error>> {
        if row_index >= self.data.len() {
            return Err(Box::new(OutIndexRowError));
        }

        let ret : String = self.data[row_index].try_get(col_index)?;
        Ok(ret)
    }
}


pub struct PgUploader {
    client : postgres::Client
}

impl PgUploader {
    pub(super) fn new(c : postgres::Client) -> Self {
        return PgUploader {
            client : c
        }
    }

    pub fn execute(&mut self, query : &'_ str, param : &'_ [&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Box<dyn Error>> {
        let ret = self.client.execute(query, param)?;
        Ok(ret)
    }

    pub fn query<T, F>(&mut self, 
        query : &'_ str, param : &'_ [&(dyn postgres::types::ToSql + Sync)]
    ,data_gen_fn : F) -> Result<Vec<T>, Box<dyn Error>> where F : Fn(PgRows) -> Result<Vec<T>, Box<dyn Error>> {
        let ret = self.client.query(query, param)?;

        let data = data_gen_fn(PgRows{data : ret})?;
        
        Ok(data)
    }
}