pub mod redis;

use std::error::Error;

pub trait Collector<T,R> {
    fn run_collect(&mut self) -> Result<(), Box<dyn Error>>;
    fn get_cmd(&self) -> R;
}