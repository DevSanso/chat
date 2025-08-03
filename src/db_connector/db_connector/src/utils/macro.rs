macro_rules! enum_option_get_only_one {
    ($enum_name:ident, $sub_enum_name:ident, $body:expr) => {
        {
            use common_rs::err::core::{COMMON_ERROR_CATEGORY, PARSING_ERROR};
            use common_rs::err::create_error;
            match $body {
                Some($enum_name::$sub_enum_name(c)) => Ok(c),
                _ => create_error(COMMON_ERROR_CATEGORY, PARSING_ERROR, format!("enum:{}, sub:{}", stringify!($enum_name), stringify!($sub_enum_name))).as_error(),
            }
        }
    };
}

pub (crate) use enum_option_get_only_one;

macro_rules! enum_get_only_one {
    ($enum_name:ident, $sub_enum_name:ident, $body:expr) => {
        {
            use common_rs::err::core::{COMMON_ERROR_CATEGORY, PARSING_ERROR};
            use common_rs::err::create_error;

            match $body {
                $enum_name::$sub_enum_name(c) => Ok(c),
                _ => create_error(COMMON_ERROR_CATEGORY, PARSING_ERROR, format!("enum:{}, sub:{}", stringify!($enum_name), stringify!($sub_enum_name))).as_error(),
            }
        }
    };
}

pub (crate) use enum_get_only_one;

macro_rules! enum_get_only_one_ref {
    ($enum_name:ident, $sub_enum_name:ident, $body:expr) => {
        {
            use common_rs::err::core::{COMMON_ERROR_CATEGORY, PARSING_ERROR};
            use common_rs::err::create_error;

            match &$body {
                $enum_name::$sub_enum_name(c) => Ok(c.clone()),
                _ => create_error(COMMON_ERROR_CATEGORY, PARSING_ERROR, format!("enum:{}, sub:{}", stringify!($enum_name), stringify!($sub_enum_name))).as_error(),
            }
        }

    };
}

pub (crate) use enum_get_only_one_ref;