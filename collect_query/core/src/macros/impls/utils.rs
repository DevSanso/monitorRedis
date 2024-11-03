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

#[macro_export]
macro_rules! utils_new_error {
    ($category : ident, $err_structure:ident, $msg :expr) => {
            {
                use core::errs;
                use core::func;
                Err(Box::new(errs::$category::$err_structure::new(format!("{} [{}:{}] : {}", func!(), file!(), line!(), $msg))))
            }
    };
}

#[macro_export]
macro_rules! utils_inherit_error {
    ($category : ident, $err_structure:ident, $msg :expr, $source_err : expr) => {
        {
            use core::errs;
            use core::func;
            let mut temp = Box::new(errs::$category::$err_structure::new(format!("{} [{}:{}] : {}", func!(), file!(), line!(), $msg)));
            let temp2 = $source_err;
            temp.push_message(&temp2);
            Err(temp)
        }
    };
}

macro_rules! utils_new_error_crate {
    ($category : ident, $err_structure:ident, $msg :expr) => {
            {
                use crate::errs;
                use crate::func;
                Err(Box::new(errs::$category::$err_structure::new(format!("{} [{}:{}, {}] : {}", func!(), file!(), line!(), module_path!(), $msg))))
            }
    };
}

pub(crate) use utils_new_error_crate;