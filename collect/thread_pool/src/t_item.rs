use std::error::Error;

use crate::TFunc;

pub type TItem<T : 'static  + Sync + Send> = (Option<T>, &'static TFunc<T>); 

