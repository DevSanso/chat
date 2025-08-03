mod config;
mod constant;
mod server;
mod args;
mod entry;
mod utils;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use clap::Parser;

use common_rs::init;
use common_rs::err::core::*;
use common_rs::db::{CommonSqlConnectionInfo, CommonSqlConnectionPool, create_common_sql_pool, DatabaseType};

fn create_db_pool(dbtype : &'_ str, config : &config::DbConfig) -> Result<CommonSqlConnectionPool, Box<dyn std::error::Error>> {
    let info = CommonSqlConnectionInfo {
        addr : format!("{}:{}", config.ip, config.port),
        db_name : config.dbname.clone(),
        user : config.user.clone(),
        password : config.password.clone(),
        timeout_sec : config.timeout_sec,
    };
    
    match dbtype {
        "POSTGRES" => Ok(create_common_sql_pool(DatabaseType::POSTGRES(info),config.dbname.clone(), config.max_size)),
        "SCYLLA" => Ok(create_common_sql_pool(DatabaseType::SCYLLA(vec![info]),config.dbname.clone(), config.max_size)),
        "DUCKDB" => Ok(create_common_sql_pool(DatabaseType::DUCKDB(info),config.dbname.clone(), config.max_size)),
        _ => create_error(COMMON_ERROR_CATEGORY, NO_SUPPORT_ERROR, format!("{}", dbtype)).as_error()
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let process_args = args::Args::parse();
    let config = config::read_config(&process_args.config)?;

    init::signal::init_once();
    init::logger::init_once(process_args.log_level.as_str(), Some(process_args.log_file.as_str()))?;

    let kill_switch = Arc::new(Mutex::new(false));

    let server_config = server::tcpd::TcpServerConfig {
        kill_switch : Arc::clone(&kill_switch),
        ip : config.server.ip,
        port : config.server.port,
        max_thread_size : config.server.thread_count,
        db_pool : create_db_pool(process_args.database.as_str(), &config.db)?,
        action : process_args.action.clone()
    };
    
    let tcp_server = server::tcpd::TcpServer::new(server_config)?;
    let job = tcp_server.start_service_async();

    loop {
        if common_rs::signal::is_set_signal(common_rs::signal::SIGINT) {
            *kill_switch.lock().unwrap() = true;
            break;
        }

        thread::sleep(time::Duration::from_secs(10));
    }

    job.join().map_err(|_| {
        let e : Result<(), Box<dyn std::error::Error>> = create_error(COMMON_ERROR_CATEGORY, CRITICAL_ERROR, "".to_string()).as_error();
        e.err().unwrap()
    })?;

    Ok(())
}
