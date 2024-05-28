use std::error::Error;
use std::fmt::Display;

use core::impl_error_structure;

impl_error_structure!(BuilderPgPoolNoneErr, "not register PgPool, check code");
