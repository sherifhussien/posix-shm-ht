mod ipc;
mod shmem;
mod sem;

use std::{thread, time};

use log::{info, warn};
use env_logger::Env;

use utils::{serliaize, deserialize};
use utils::message::{self, Message};
use ipc::IPC;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Client ***********************");    
    
    let mut ipc: IPC = IPC::new();
    
     match ipc.init() {
        Ok(_) => info!("IPC >> initialized successfully"),
        Err(err) => warn!("IPC >> init error: {}", err),
    }
    
    /* test */
    thread::sleep(time::Duration::from_secs(5));
    test_1(&ipc);
    /* test */
        
    match ipc.clean() {
        Ok(_) => info!("IPC >> cleaned successfully"),
        Err(err) => warn!("IPC >> clean error: {}", err),
    }
}

fn test_1(ipc: &IPC) {
    let message = Message {
        typ: message::CLIENT_GET,
        content: serliaize("key1")
    };

    match ipc.write(message) {
        Ok(_) => info!(">> wrote message"),
        Err(err) => warn!(">> can't write message: {}", err),
    }
}