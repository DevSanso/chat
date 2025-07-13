
use serde::{Deserialize};

#[derive(Deserialize)]
struct ServerConfig {
    #[serde( alias = "ip")]
    ip : String,
    #[serde( alias = "port")]
    port : u32,
    #[serde( alias = "thread_count")]
    thread_count : usize
}

#[derive(Deserialize)]
struct DbConfig {
    #[serde( alias = "ip")]
    ip : String,
    #[serde( alias = "port")]
    port : u32,
    #[serde( alias = "user")]
    user : String,
    #[serde( alias = "password")]
    password : String,
    #[serde( alias = "dbname")]
    dbname : String
}

#[derive(Deserialize)]
struct Config {
    #[serde( alias = "server_config")]
    server : ServerConfig,
    #[serde( alias = "db_config")]
    db : DbConfig
}