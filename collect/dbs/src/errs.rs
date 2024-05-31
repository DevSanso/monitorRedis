use std::error::Error;
use std::fmt::Display;

use core::impl_error_structure;

impl_error_structure!(NotMatchArgsLenError, "query bound args count not mathcing");
impl_error_structure!(OutIndexRowError, "Out Index Select Row Error");
impl_error_structure!(RowIdxNotExistError, "Not Exists col data in this row");
impl_error_structure!(RowGetDataInternalError, "row data cast internal error");
impl_error_structure!(arg, NotMatchTypeError, "not match type");