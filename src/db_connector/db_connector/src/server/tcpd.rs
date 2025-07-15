use std::cell::RefCell;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use common::common_make_err;
use common_conn::{CommonSqlConnectionPool, CommonSqlConnection};
use common::logger;
use idl::protos::dbconn::DbConnRequest;
use idl::protos::dbconn::DbConnResponse;
use crate::entry::proto::search_proto_entry;
//type proto_entry_fn = Box<dyn Fn(DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> + Send + 'static>;


pub struct TcpServerConfig {
    pub kill_switch : Arc<Mutex<bool>>,
    pub db_pool : CommonSqlConnectionPool,

    pub ip : String,
    pub port : u32,
    pub max_thread_size : usize,
    pub action : String
}
pub struct TcpServer {
    listen : TcpListener,
    kill_switch : Arc<Mutex<bool>>,
    max_thread_size : usize,
    active_thread_size : Arc<Mutex<usize>>,
    action : String,
    db_pool : CommonSqlConnectionPool
}

impl TcpServer {
    pub fn new(config : TcpServerConfig) -> Result<Self, Box<dyn Error>> {
        let listen  = TcpListener::bind(format!("{}:{}", config.ip, config.port).as_str()).map_err(|e| {
            let err : Result<(), Box<dyn std::error::Error>> = common_make_err!(system, ApiCallError,
                 "[err:{}][ip:{}] [port:{}]", e, config.ip, config.port);
            err.unwrap_err()
        })?;

        Ok(TcpServer { 
            listen: listen,
            kill_switch : config.kill_switch,
            max_thread_size : config.max_thread_size,
            active_thread_size : Arc::new(Mutex::new(0)),
            action : config.action,
            db_pool : config.db_pool
        })
    }

    fn is_kill_server(&mut self) -> bool {
         let is_kill_ret = self.kill_switch.lock().map_err(|x| {
            let err : Result<(), Box<dyn Error>> = common_make_err!(system, ApiCallError, 
                "{}", x);
            err.unwrap_err()
        });

        if is_kill_ret.is_err() {
            logger::error!("{}", is_kill_ret.unwrap_err());
            return false;
        }

        if *is_kill_ret.unwrap() {
            return false;
        }

        true
    }

    fn accept_client(&mut self) -> Option<(TcpStream, SocketAddr)> {
        let accept = self.listen.accept();

        if accept.is_err() {
            let err : Result<(), Box<dyn Error>> = common_make_err!(system, ApiCallError, 
                "{}", accept.unwrap_err());
            
            logger::error!("{}", err.unwrap_err());
            None
        }
        else {
            let (c,addr)= accept.unwrap();
            let set_ret = c.set_read_timeout(Some(time::Duration::new(3,0)));

            if set_ret.is_err() {
                let err : Result<(), Box<dyn Error>> = common_make_err!(system, ApiCallError, 
                    "{}", set_ret.unwrap_err());

                logger::error!("{}", err.unwrap_err());
                None
            }else {
                Some((c,addr))
            }

        }
    }

    fn get_proto_data_from_client(c : &mut TcpStream) -> Result<DbConnRequest, Box<dyn Error>> {
        let mut buf = Vec::new();
        loop {
            let mut byte_buf = [0 as u8;4096];
            let read_byte = c.read(&mut byte_buf).map_err(|e| {
                let err : Result<(), Box<dyn Error>> = common_make_err!(system, ApiCallError, 
                    "{}", e);
                err.unwrap_err()
            })?;

            if read_byte == 0 {break;}
            buf.extend_from_slice(&byte_buf[..read_byte]);
        }

        let data = idl::decode_protobuf::<DbConnRequest>(buf.as_slice()).map_err(|e| {
            let err : Result<(), Box<dyn Error>> = common_make_err!(system, ApiCallError, 
                "{}", e);
            err.unwrap_err()
        })?;

        Ok(data)
    }

    fn send_proto_data_from_client(c : &mut TcpStream, data : DbConnResponse) -> Result<(), Box<dyn Error>> {
        let send_data = idl::encode_protobuf(data)?;

        c.write(send_data.as_slice()).map_err(|e| {
            let err : Result<(), Box<dyn Error>> = common_make_err!(system, ApiCallError, 
                "{}", e);
            err.unwrap_err()
        })?;

        Ok(())
    }

    fn start_entry(action :&'_ str, pool : CommonSqlConnectionPool, mut client : TcpStream) {
        let data = Self::get_proto_data_from_client(&mut client);
        if data.is_err() {
            logger::error!("{}", data.unwrap_err());
            return;
        }

        let entry = search_proto_entry(action, data.as_ref().unwrap());
        if entry.is_err() {
            let e= entry.err().unwrap();
            logger::error!("{}", e);
            return;
        }
        let conn = pool.get_owned(());
        if conn.is_err() {
            let e= conn.err().unwrap();
            logger::error!("{}", e);
            return;
        }

        let mut conn_ret = conn.unwrap();
        let real_conn = conn_ret.get_value();
        let entry_ret = entry.unwrap()(real_conn, data.unwrap());

        let send_ret = Self::send_proto_data_from_client(&mut client, entry_ret.unwrap());
        if send_ret.is_err() {
            logger::error!("{}", send_ret.unwrap_err());
            conn_ret.dispose();
        }
    }

    fn check_thread_count(&self) -> bool {
        let act_size = self.active_thread_size.lock().unwrap();
        return self.max_thread_size < *act_size;
    }

    fn server_main(&mut self) {
        let cur_thread = thread::current();
        let t_name = cur_thread.name().take().unwrap();

        thread::scope(|ctl| {
            loop {
                if self.is_kill_server() {
                    break;
                }

                let client_opt = self.accept_client();
                if client_opt.is_none() {
                    continue;
                }
                let (client, client_addr) = client_opt.unwrap();
                logger::info!("aceept client [thread:{}] [addr:{}]", t_name, client_addr.ip());

                if !self.check_thread_count() {
                    let err : Result<(), Box<dyn Error>> = common_make_err!(system, LimitError, "");
                    logger::error!("{}", err.unwrap_err());
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                }

                let active_status = Arc::clone(&self.active_thread_size);
                let action = self.action.clone();
                let p = Arc::clone(&self.db_pool);

                ctl.spawn(|| {
                    let active = active_status;
                    {
                        *active.lock().unwrap() += 1;
                    }

                    let client = client;
                    let action = action;
                    let pool = p;

                    Self::start_entry(action.as_str(), pool, client);

                    {
                        *active.lock().unwrap() -= 1;
                    }
                });
            }
        });
    }

    pub fn start_service_async(mut self) -> thread::JoinHandle<()> {
        let job =thread::spawn(move || {self.server_main();});

        job
    }
}