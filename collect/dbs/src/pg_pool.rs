use std::error::Error;
use std::sync::Arc;

use postgres::{self, Row, Transaction};
use log::*;

use core::structure::owned_pool::{OwnedPool, PoolItemOwned};
use core::utils_inherit_error;

pub type PgPoolAlias =  Arc<OwnedPool<PgConnecter, ()>>;

pub fn new_pg_pool(name : String, url : String, max_size : usize) -> PgPoolAlias {
    OwnedPool::new(name, Box::new(|_ : () | {
        match postgres::Client::connect(url.as_str(), postgres::NoTls) {
            Ok(client) => Some(PgConnecter::new(client)),
            Err(c) =>  {
                trace!("PgPool - gen : {}", c);
                None
            }
        }
    }), max_size)
}

pub struct PgTrans<'a> {
    raw : Transaction<'a>
}

impl<'a> PgTrans<'a> {
    pub fn rollback(self) -> Result<(), impl Error> {
        self.raw.rollback()
    }
    pub fn execute(&mut self, query : &'_ str, param : &'_ [&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Box<dyn Error>> {
        let ret = self.raw.execute(query, param);

        if ret.is_err() {
            return utils_inherit_error!(connection, CommandRunError, "execute[trans]", ret.err().unwrap());
        }

        Ok(ret.unwrap())
    }
    pub fn commit(self) -> Result<(), impl Error> {
        self.raw.commit()
    }
}

pub struct PgRows {
    rs : Vec<Row>
}

impl PgRows {
    pub fn new(rs : Vec<Row>) -> Self {
        PgRows { rs: rs }
    }
    pub fn get_f64_data(&self, row_idx : usize, col_idx : usize) {
        let r = self.rs.get(row_idx).unwrap();
        let ret : f64 = r.try_get(col_idx).unwrap();
    }
    pub fn get_i64_data(&self, row_idx : usize, col_idx : usize) {
        let r = self.rs.get(row_idx).unwrap();
        let ret : i64 = r.try_get(col_idx).unwrap();
    }
    pub fn get_str_data(&self, row_idx : usize, col_idx : usize) {
        let r = self.rs.get(row_idx).unwrap();
        let ret : &str = r.try_get(col_idx).unwrap();
    }
}

pub struct PgConnecter {
    client : postgres::Client
}

impl PgConnecter {
    pub(super) fn new(c : postgres::Client) -> Self {
        return PgConnecter {
            client : c
        }
    }
}
pub trait PgSelecter {
    fn query(&mut self, query : &'_ str, param : &'_ [&(dyn postgres::types::ToSql + Sync)]) -> Result<PgRows, Box<dyn Error>>;
}

impl PgSelecter for PgConnecter {
    fn query(&mut self, query : &'_ str, param : &'_ [&(dyn postgres::types::ToSql + Sync)]) -> Result<PgRows, Box<dyn Error>> {
        let rs = self.client.query(query, param);

        if rs.is_err() {
            return utils_inherit_error!(connection, CommandRunError, "query", rs.err().unwrap());
        }

        Ok(PgRows::new(rs.unwrap()))
    }
}

pub trait PgUploader {
    fn execute(&mut self, query : &'_ str, param : &'_ [&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Box<dyn Error>>;

    fn trans(&mut self) -> Result<PgTrans<'_>, Box<dyn Error>>;
}

impl PgUploader for PgConnecter {
    fn execute(&mut self, query : &'_ str, param : &'_ [&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Box<dyn Error>> {
        let ret = self.client.execute(query, param);
        
        if ret.is_err() {
            return utils_inherit_error!(connection, CommandRunError, "execute", ret.err().unwrap());
        }

        Ok(ret.unwrap())
    }

    fn trans(&mut self) -> Result<PgTrans<'_>, Box<dyn Error>>{
        let t = self.client.transaction();
        
        if t.is_err() {
            return utils_inherit_error!(connection, ConnectionApiCallError, "transcation", t.err().unwrap());
        }
        
        Ok(PgTrans{ raw : t.unwrap()})
    }
}





