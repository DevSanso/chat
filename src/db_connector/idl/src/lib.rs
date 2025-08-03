pub mod protos;

use std::error::Error;
use prost;

use common_rs::err::create_error;
use common_rs::err::core::{COMMON_ERROR_CATEGORY, PARSING_ERROR};

pub fn decode_protobuf<T : prost::Message + std::default::Default>(data : &'_ [u8]) -> Result<T, Box<dyn Error>> {
    match T::decode(data) {
        Ok(ok) => Ok(ok),
        Err(e) => create_error(COMMON_ERROR_CATEGORY, PARSING_ERROR, format!("{}", e)).as_error()
    }
}

pub fn encode_protobuf<T : prost::Message + std::default::Default>(data : T) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes_buf = Vec::new();

    match data.encode(&mut bytes_buf) {
        Ok(()) => Ok(bytes_buf),
        Err(e) => create_error(COMMON_ERROR_CATEGORY, PARSING_ERROR, format!("{}", e)).as_error()
    }
}