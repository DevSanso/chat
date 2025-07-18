#[macro_export]
macro_rules! enum_option_get_only_one {
    ($enum_name:ident, $sub_enum_name:ident, $body:expr) => {
        match $body {
            Some($enum_name::$sub_enum_name(c)) => Ok(c),
            _ => common_make_err!(data, ParsingError, "enum:{}, sub:{}", stringify!($enum_name), stringify!($sub_enum_name)),
        }
    };
}

#[macro_export]
macro_rules! enum_get_only_one {
    ($enum_name:ident, $sub_enum_name:ident, $body:expr) => {
        match $body {
            $enum_name::$sub_enum_name(c) => Ok(c),
            _ => common_make_err!(data, ParsingError, "enum:{}, sub:{}", stringify!($enum_name), stringify!($sub_enum_name)),
        }
    };
}

#[macro_export]
macro_rules! enum_get_only_one_ref {
    ($enum_name:ident, $sub_enum_name:ident, $body:expr) => {
        match &$body {
            $enum_name::$sub_enum_name(c) => Ok(c.clone()),
            _ => common_make_err!(data, ParsingError, "enum:{}, sub:{}", stringify!($enum_name), stringify!($sub_enum_name)),
        }
    };
}