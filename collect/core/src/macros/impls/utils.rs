#[macro_export]
macro_rules! utils_new_error {
    ($category : ident, $err_structure:ident, $msg :expr) => {
            {
                use core::errs;
                Err(Box::new(errs::$category::$err_structure::new(format!("(pos:{}:{}:{}, hint:{})",file!(), module_path!(), line!(), $msg))))
            }
    };
}

#[macro_export]
macro_rules! utils_inherit_error {
    ($category : ident, $err_structure:ident, $msg :expr, $source_err : expr) => {
        {
            use core::errs;
            Err(Box::new(errs::$category::$err_structure::new(format!("(pos:{}:{}:{}, hint:{}, origin:{})",file!(), module_path!(), line!(), $msg, $source_err.to_string()))))
        }
    };
}

#[macro_export]
macro_rules! func {
    () => {
        {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            &name[..name.len() - 3]
        }
    };
}

macro_rules! utils_new_error_crate {
    ($category : ident, $err_structure:ident, $msg :expr) => {
            {
                use crate::errs;
                Err(Box::new(errs::$category::$err_structure::new(format!("{} [{}:{}, {}] : {}", crate::func!(), file!(), line!(), module_path!(), $msg))))
            }
    };
}

pub(crate) use utils_new_error_crate;