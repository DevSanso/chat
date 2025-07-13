pub mod protos;

use std::error::Error;
use prost;

use common::common_make_err;

pub fn decode_protobuf<T : prost::Message + std::default::Default>(data : &'_ [u8]) -> Result<T, Box<dyn Error>> {
    match T::decode(data) {
        Ok(ok) => Ok(ok),
        Err(e) => common_make_err!(data, ParsingError, "{}", e)
    }
}

pub fn encode_protobuf<T : prost::Message + std::default::Default>(data : T) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes_buf = Vec::new();

    match data.encode(&mut bytes_buf) {
        Ok(()) => Ok(bytes_buf),
        Err(e) => common_make_err!(data, ParsingError, "{}", e)
    }
}