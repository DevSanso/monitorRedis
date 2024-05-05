use crate::impl_error_structure;
use std::error::Error;
use std::fmt::Display;


impl_error_structure!(CriticalError, "Critical Error");
impl_error_structure!(GenResultIsNoneError, "Critical Error");
impl_error_structure!(MaxSizedError, "Can't Make Object, because size is full");

pub mod pool;
