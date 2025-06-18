pub mod define;

macro_rules! impl_error {
    ($category:ident ,$name : ident, $message:expr, $descr : expr) => {
        #[derive(Debug)]
        pub struct $name(pub String /* sub message*/);

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let _ = write!(f, "{}:{} = {},  {}", stringify!($category), stringify!($name), stringify!($message), self.0);
                std::fmt::Result::Ok(())
            }
        }

        impl Error for $name  {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                None
            }
        
            fn description(&self) -> &str {
                stringify!($descr)
            }
        
            fn cause(&self) -> Option<&dyn Error> {
                self.source()
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

            use crate::err::impl_error;

            $(impl_error!($name, $err_name, $message, $descr);)*
        }
    }
}

pub(crate) use impl_error;
pub(crate) use impl_err_mod;

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
pub use func;

#[macro_export]
macro_rules! common_make_err {
    ($category:ident ,$name : ident, $($arg:tt)+) => {{
        use common::err::func;
        use common::err::define::*;

        Err(Box::new($category::$name(format!("{} [{}:{}] : {}\n", func!(), file!(), line!(), format!($($arg)+)))))
    }};
}


macro_rules! make_err_crate {
    ($category:ident ,$name : ident, $($arg:tt)+) => {{
        use crate::err::func;
        use crate::err::define::*;

        Err(Box::new($category::$name(format!("{} [{}:{}] : {}\n", func!(), file!(), line!(), format!($($arg)+)))))
    }};
}
pub use common_make_err;
pub(crate) use make_err_crate;

