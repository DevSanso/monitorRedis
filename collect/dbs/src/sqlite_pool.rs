use rusqlite::{Connection, OpenFlags};

use core::structure::pool::{Pool, PoolItem};
use std::error::Error;

pub struct SqlitePool {
    p : Pool<Connection, String>,
    file_path : String
}

impl SqlitePool {
    pub fn new(file_path : String) -> Self {
        return SqlitePool { p: Pool::new(Box::new(Self::gen), 1) , file_path}
    }

    pub fn get(&mut self) -> Result<PoolItem<Connection>, Box<dyn Error>> {
        self.p.get(self.file_path.clone())
    }

    fn gen(path : String) -> Option<Connection> {
        let c = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY);
        if c.is_err() {
            return None;
        }

        Some(c.unwrap())
    }
}