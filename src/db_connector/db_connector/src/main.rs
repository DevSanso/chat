mod config;
mod constant;
mod server;
mod args;
mod entry;

use std::sync::{Arc, Mutex};
use std::{thread, time};

use clap::Parser;

use common::init;
use common::common_make_err;
use duckdb_conn::create_duckdb_conn_pool;
use postgres_conn::create_pg_conn_pool;
use scylla_conn::create_scylla_conn_pool;
use common_conn::{CommonSqlConnectionPool, CommonSqlConnectionInfo};

fn create_db_pool(dbtype : &'_ str, config : &config::DbConfig) -> Result<CommonSqlConnectionPool, Box<dyn std::error::Error>> {
    let info = CommonSqlConnectionInfo {
        addr : format!("{}:{}", config.ip, config.port),
        db_name : config.dbname.clone(),
        user : config.user.clone(),
        password : config.password.clone(),
        timeout_sec : config.timeout_sec,
    };
    
    match dbtype {
        "POSTGRES" => Ok(create_pg_conn_pool("pg".to_string(), info, config.max_size)),
        "SCYLLA" => Ok(create_scylla_conn_pool("scylla".to_string(), vec![info], config.max_size)),
        "DUCKDB" => Ok(create_duckdb_conn_pool("duck".to_string(), info, config.max_size)),
        _ => common_make_err!(system, NoSupportError, "{}", dbtype)
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
        if common::signal::is_set_signal(common::signal::SIGINT) {
            *kill_switch.lock().unwrap() = true;
            break;
        }

        thread::sleep(time::Duration::from_secs(10));
    }

    job.join().map_err(|_| {
        let e : Result<(), Box<dyn std::error::Error>> = common_make_err!(system, CriticalError,"");
        e.err().unwrap()
    })?;

    Ok(())
}
