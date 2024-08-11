
macro_rules! impl_error {
    ($category:ident ,$name : ident, $message:expr, $descr : expr) => {
        
        #[derive(Debug)]
        pub struct $name(&'static str /* message(description) */,  Vec<String> /* output list*/);

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for item in &self.1 {
                    write!(f, "{}\n",item)?;
                }

                std::fmt::Result::Ok(())
            }
        }

        impl Error for $name  {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                None
            }
        
            fn description(&self) -> &str {
                self.0
            }
        
            fn cause(&self) -> Option<&dyn Error> {
                self.source()
            }
        }

        impl $name {
            pub fn new(sub_msg : String) -> Self {
                let mut ret = $name($message, Vec::new());
                ret.1.push(format!("{}:{}[{}] : {}",stringify!($category), stringify!($name), $message, sub_msg));

                ret
            }

            pub fn push_message(&mut self, msg : &'_ dyn Display) {
                self.1.push(format!("{}", msg));
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


