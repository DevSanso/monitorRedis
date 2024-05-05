use std::error::Error;

use postgres;

use core::structure::pool::{Pool, PoolItem};

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
}