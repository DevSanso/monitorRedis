use crate::impl_error_structure;
use std::error::Error;
use std::fmt::Display;

impl_error_structure!(CriticalError, "Critical Error");
impl_error_structure!(arg, GenResultIsNoneError, "Critical Error");
impl_error_structure!(arg, MaxSizedError, "Can't Make Object, because size is full");