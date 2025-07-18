pub mod collection;
pub mod err;
pub mod init;
pub mod utils;

pub mod logger {
    pub use log::debug;
    pub use log::error;
    pub use log::info;
    pub use log::trace;

    use crate::init;

    pub fn get_is_trace_level() -> bool {
        unsafe {
            return init::logger::LOGGER_FILE_LEVEL_IS_TRACE;
        }
    }
}

pub mod signal {
    pub use crate::init::signal::SIGABRT;
    pub use crate::init::signal::SIGBUS;
    pub use crate::init::signal::SIGINT;
    pub use crate::init::signal::SIGPIPE;

    pub fn is_set_signal(num : i32) -> bool {
        crate::init::signal::is_set_signal(num)
    }
}