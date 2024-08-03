use std::{any::Any, error::Error, fmt::Debug};
use std::sync::Arc;

use redis::{Client, Cmd, Commands, FromRedisValue, Value};

use log::*;

use core::structure::owned_pool::OwnedPool;
use core::utils_inherit_error;
use core::utils_new_error;

pub type RedisPoolAlias =  Arc<OwnedPool<RedisRequester, ()>>;

pub fn new_redis_pool(name : String, url : String, max_size : usize) -> RedisPoolAlias {
    OwnedPool::new(name, Box::new( move|_ : () | {
        match Client::open(url.clone()) {
            Ok(client) => Some(RedisRequester::new(client)),
            Err(c) => {
                trace!("RedisPool - gen : {}", c);
                None
            }
        }
    }), max_size)
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
        let param_count = command.matches("?").count();
        if args.len() != param_count {
            return utils_new_error!(connection, NotMatchArgsLenError, format!("{}:{}", args.len(), param_count));
        }
        let mut v: Vec<String> = Vec::new();

        let token = command.as_bytes();   
        let mut str_buf = String::new();
        let mut trigger_str = false;
        let mut args_index = 0;

        for idx in 0..token.len() {
            if token[idx] == b'\"' && !trigger_str {
                trigger_str = true;
                continue;
            }

            if token[idx] != b'\"' && trigger_str {
                str_buf.push(char::from(token[idx]));
                continue;
            }

            if token[idx] == b'\"' && trigger_str {
                trigger_str = false;
                v.push(str_buf.clone());

                str_buf.clear();
                continue;
            }

            if token[idx] == b' ' {
                if str_buf.len() > 0 {
                    v.push(str_buf.clone());
                    str_buf.clear();
                    continue;
                }
                else {
                    continue;
                }
            }

            if token[idx] == b'?' {
                str_buf.push_str(args[args_index]);
                args_index += 1;
                continue;
            }

            str_buf.push(char::from(token[idx]));     
        }

        if str_buf.len() > 0 {
            v.push(str_buf.clone());
        }
        
        Ok(v)
    }

    fn bulk_to_string(v : &Vec<Value>) -> Result<String, Box<dyn Error>> {
        let mut ret = String::from("");

        for item in v {
            let cast : String = match item {
                Value::Bulk(b) => Self::bulk_to_string(b)?,
                Value::Nil => String::from("\n"),
                Value::Int(i) => format!("{}\n", i),
                Value::Status(s) => format!("{}\n", s.as_str()),
                Value::Okay => String::from("\n"),
                Value::Data(bin) =>
                {
                    let temp = String::from_utf8(bin.clone());
                    
                    if temp.is_err() {

                        return utils_new_error!(data, EncodingCastError, "");
                    }
                    else {
                        let mut s = temp.unwrap();
                        s.push_str("\n");
                        s
                    }
                }
            };

            ret.push_str(cast.as_str());
        }

        Ok(ret)
    }
    pub fn ping(&mut self) -> Result<(), Box<dyn Error>> {
        let mut cmd = Cmd::new();
        cmd.arg("PING");

        let mut conn = {
            let c = self.client.get_connection();
            if c.is_err() {
                return utils_inherit_error!(connection, GetConnectionFailedError, "", c.err().unwrap());
            }
            c.unwrap()
        };

        let ret = cmd.query::<()>(&mut conn);
        if ret.is_err() {
            return utils_inherit_error!(connection, CommandRunError, "", ret.err().unwrap());
        }

        Ok(())
    }
    pub fn set_app_name(&mut self, application_name : &'_ str) -> Result<(), Box<dyn Error>> {
        let mut cmd = Cmd::new();
        cmd.arg("CLIENT");
        cmd.arg("SETNAME");
        cmd.arg(application_name);

        let mut conn = {
            let c = self.client.get_connection();
            if c.is_err() {
                return utils_inherit_error!(connection, GetConnectionFailedError, "", c.err().unwrap());
            }
            c.unwrap()
        };

        let ret : Value = match cmd.query(&mut conn) {
            Ok(ok) => ok,
            Err(err) => return utils_inherit_error!(connection, CommandRunError, "", err)
        };
        
        Ok(())
    }
    pub fn run_command(&mut self, command : &'_ str, args : &'_ [&'_ str]) -> Result<String, Box<dyn Error>> {
        let mut cmd = Cmd::new();
        let split_cmd = self.parsing_args(command, args)?;

        for c in split_cmd {
            cmd.arg(c);
        }
        
        let mut conn = {
            let c = self.client.get_connection();
            if c.is_err() {
                return utils_inherit_error!(connection, GetConnectionFailedError, "", c.err().unwrap());
            }
            c.unwrap()
        };
        
        let ret : Value = match cmd.query(&mut conn) {
            Ok(ok) => ok,
            Err(err) => return utils_inherit_error!(connection, CommandRunError, "", err)
        };

        let s = match ret {
            Value::Nil => return utils_new_error!(fetch, NilDataError, ""),
            Value::Okay => String::from(""),
            Value::Status(s) => s,
            Value::Bulk(v) => Self::bulk_to_string(&v)?,
            Value::Data(b) => String::from_utf8(b)?,
            Value::Int(i) => i.to_string()
        };
        Ok(s)
    }
}