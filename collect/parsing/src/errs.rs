use std::error::Error;
use std::fmt::Display;
use core::impl_error_structure;

impl_error_structure!(CantCastTupleError, "this value cant cast tuple");
impl_error_structure!(arg, CantMappingValueError, "cant mapping this data type");