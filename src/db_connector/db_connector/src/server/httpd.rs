use std::error::Error;
use std::fs::read;
use std::io::Read;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc,Mutex};
use std::{thread, time};

use common::common_make_err;
use common::logger;
use idl::protos::dbconn::DbConnRequest;

struct TcpServerConfig {
    ip : String,
    port : u32,
    kill_switch : Arc<Mutex<bool>>,
    proto_entry_fn : Option<Box<dyn Fn(DbConnRequest) -> Result<(), Box<dyn Error>> + Send + 'static>>
}
struct TcpServer {
    listen : TcpListener,
    proto_entry_fn : Option<Box<dyn Fn(DbConnRequest) -> Result<(), Box<dyn Error>> + Send + 'static>>,
    kill_switch : Arc<Mutex<bool>>
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
            proto_entry_fn : config.proto_entry_fn 
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

        let data = idl::parsing_protobuf::<DbConnRequest>(buf.as_slice()).map_err(|e| {
            let err : Result<(), Box<dyn Error>> = common_make_err!(system, ApiCallError, 
                "{}", e);
            err.unwrap_err()
        })?;

        Ok(data)
    }

    pub fn start_service_async(mut server : TcpServer) -> thread::JoinHandle<()> {
        let job =thread::spawn(move || {
            let entry = server.proto_entry_fn.take().unwrap();
            let cur_thread = thread::current();
            let t_name = cur_thread.name().take().unwrap();

            loop {
                if server.is_kill_server() {
                    break;
                }

                let client_opt = server.accept_client();
                if client_opt.is_none() {
                    continue;
                }
                let (mut client, client_addr) = client_opt.unwrap();
                logger::info!("aceept client [thread:{}] [addr:{}]", t_name, client_addr.ip());

                let data = Self::get_proto_data_from_client(&mut client);
                if data.is_err() {
                    logger::error!("{}", data.unwrap_err());
                    continue;
                }

                let entry_ret = entry(data.unwrap());
                if entry_ret.is_err() {
                    logger::error!("{}", entry_ret.unwrap_err());
                    continue;
                }
            }
        });

        job
    }
}