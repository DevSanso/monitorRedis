macro_rules! impl_error {
    ($name : ident, $message:expr, $descr : expr) => {
        
        #[derive(Debug)]
        pub struct $name;

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!($name))
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

    };
}

#[macro_export]
macro_rules! impl_err_mod {
    ($name:ident, [$((
        $err_name:ident, $message:expr, $descr:expr)),*
    ]) => {
        pub mod $name {
            use std::error::Error;
            use std::fmt::Display;
            use std::fmt::Debug;

            $(impl_error!($err_name, $message, $descr);)*

        }
    }
}



