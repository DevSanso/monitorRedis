use crate::TFunc;

pub type TItem<T> = (Option<T>, &'static TFunc<T>); 

