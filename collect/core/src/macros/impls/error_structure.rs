#[macro_export]
macro_rules! impl_error_structure {
    ($name : ident, $descr : expr) => {
        
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