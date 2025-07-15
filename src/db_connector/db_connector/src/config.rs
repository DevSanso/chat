
use std::error::Error;

use serde::{Deserialize};
use toml;

use common::common_make_err;

#[derive(Deserialize)]
pub struct ServerConfig {
    #[serde( alias = "ip")]
    pub ip : String,
    #[serde( alias = "port")]
    pub port : u32,
    #[serde( alias = "thread_count")]
    pub thread_count : usize
}

#[derive(Deserialize)]
pub struct DbConfig {
    #[serde( alias = "ip")]
    pub ip : String,
    #[serde( alias = "port")]
    pub port : u32,
    #[serde( alias = "user")]
    pub user : String,
    #[serde( alias = "password")]
    pub password : String,
    #[serde( alias = "dbname")]
    pub dbname : String,
    #[serde( alias = "max_size")]
    pub max_size : usize,
    #[serde( alias = "timeout")]
    pub timeout_sec : u32
}

#[derive(Deserialize)]
pub struct Config {
    #[serde( alias = "server_config")]
    pub server : ServerConfig,
    #[serde( alias = "db_config")]
    pub db : DbConfig
}

pub fn read_config(path : &'_ str) -> Result<Config, Box<dyn Error>> {
    let data = std::fs::read_to_string(path).map_err(|x| {
        let e : Result<(), Box<dyn Error>> = common_make_err!(system, FileIoError, "{}", x);
        e.unwrap_err()
    })?;

    toml::from_str::<Config>(data.as_str()).map_err(|x| {
        let e :Result<Config, Box<dyn Error>> = common_make_err!(data, ParsingError, "{}", x);
        e.err().unwrap()
    })
}