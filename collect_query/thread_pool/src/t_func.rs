use std::error::Error;

pub type TFunc<T> = (dyn Fn(Option<T>) -> Result<(), Box<dyn Error>> + Sync + Send); 

