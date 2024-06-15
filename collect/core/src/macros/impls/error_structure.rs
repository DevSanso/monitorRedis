

macro_rules! impl_error {
    ($category:ident ,$name : ident, $message:expr, $descr : expr) => {
        
        #[derive(Debug)]
        pub struct $name(&'static str, &'static str, String);

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}:{} => {}", stringify!($name), self.2, $message )
            }
        }

        impl Error for $name  {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                None
            }
        
            fn description(&self) -> &str {
                $descr
            }
        
            fn cause(&self) -> Option<&dyn Error> {
                self.source()
            }
        }

        impl $name {
            pub fn new(sub_msg : String) -> Self {
                $name(stringify!($category), $message, sub_msg)
            }
        }

    };
}

macro_rules! impl_err_mod {
    ($name:ident, [$((
        $err_name:ident, $message:expr, $descr:expr)),*
    ]) => {
        pub mod $name {
            use std::error::Error;
            use std::fmt::Display;
            use std::fmt::Debug;

            use crate::macros::impl_error;

            $(impl_error!($name, $err_name, $message, $descr);)*
        }
    }
}

pub(crate) use impl_error;
pub(crate) use impl_err_mod;


