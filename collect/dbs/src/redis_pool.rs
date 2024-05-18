use std::error::Error;

use redis::{Client, Cmd};

use core::structure::pool::{Pool, PoolItem};
use crate::errs::NotMatchArgsLenError;
pub struct RedisPool {
    pool : Pool<RedisRequester, String>,
    url : String
}

impl RedisPool {
    pub fn new(url : String) -> Self {
        RedisPool {
            pool : Pool::new(Box::new(RedisPool::gen), 10),
            url
        }
    }

    fn gen(url : String) -> Option<RedisRequester> {
        match Client::open(url) {
            Ok(client) => Some(RedisRequester::new(client)),
            Err(_) => None
        }
    }

    pub fn get(&mut self) -> Result<PoolItem<RedisRequester>, Box<dyn Error>> {
        self.pool.get(self.url.clone())
    }
}

pub struct RedisRequester {
    client : Client
}

impl RedisRequester {
    pub(super) fn new(c : Client) -> Self {
        RedisRequester {
            client : c
        }
    }

    fn parsing_args(&self, command : &'_ str, args : &'_ [&'_ str]) -> Result<String,Box<dyn Error>> {
        if args.len() != command.matches("?").count() {
            return Err(Box::new(NotMatchArgsLenError));
        }

        if args.len() == 0 {
            return Ok(String::from(command));
        }
        
        let mut ret = String::new();
        
        for arg in args {
            ret = ret.replacen("?", arg, 1);
        }

        Ok(ret)
    }

    pub fn run_command(&mut self, command : &'_ str, args : &'_ [&'_ str]) -> Result<String, Box<dyn Error>> {
        let mut cmd = Cmd::new();
        cmd.arg(self.parsing_args(command, args)?);
        let ret : String = cmd.query(&mut self.client)?;
        Ok(ret)
    }
}