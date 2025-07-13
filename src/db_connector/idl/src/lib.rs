pub mod protos;

use std::error::Error;
use prost;

use common::common_make_err;

pub fn parsing_protobuf<T : prost::Message + std::default::Default>(data : &'_ [u8]) -> Result<T, Box<dyn Error>> {
    match T::decode(data) {
        Ok(ok) => Ok(ok),
        Err(e) => common_make_err!(data, ParsingError, "{}", e)
    }
}