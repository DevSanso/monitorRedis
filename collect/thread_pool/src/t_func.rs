use std::error::Error;

pub type TFunc<T : 'static> = (dyn Fn(Option<T>) -> Result<(), Box<dyn Error>> + Sync + Send); 

