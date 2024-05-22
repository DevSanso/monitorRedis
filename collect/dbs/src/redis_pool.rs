use std::error::Error;

use redis::{Client, Cmd, Commands};

use log::*;

use core::structure::pool::{Pool, PoolItem};
use crate::errs::NotMatchArgsLenError;
pub struct RedisPool {
    pool : Pool<RedisRequester, String>,
    url : String
}

impl RedisPool {
    pub fn new(ip : String, url : String) -> Self {
        RedisPool {
            pool : Pool::new(ip, Box::new(RedisPool::gen), 10),
            url
        }
    }

    fn gen(url : String) -> Option<RedisRequester> {
        match Client::open(url) {
            Ok(client) => Some(RedisRequester::new(client)),
            Err(c) => {
                trace!("RedisPool - gen : {}", c);
                None
            }
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

    fn parsing_args(&self, command : &'_ str, args : &'_ [&'_ str]) -> Result<Vec<String>,Box<dyn Error>> {
        if args.len() != command.matches("?").count() {
            return Err(Box::new(NotMatchArgsLenError));
        }
        let mut args_index = 0;
        let mut v = Vec::new();

        let mut token = command.split(" ");    

        loop {
            let t = token.next();
            if t.is_none() {break}

            let s = t.unwrap();
            if s == "?" {
                v.push(String::from(args[args_index]));
                args_index += 1;
            }else {
                v.push(String::from(s));
            }
        }

        Ok(v)
    }

    pub fn run_command(&mut self, command : &'_ str, args : &'_ [&'_ str]) -> Result<String, Box<dyn Error>> {
        let mut cmd = Cmd::new();
        let split_cmd = self.parsing_args(command, args)?;

        for c in split_cmd {
            cmd.arg(c);
        }

        let mut conn = self.client.get_connection()?;

        let ret : String = cmd.query(&mut conn)?;
        Ok(ret)
    }
}