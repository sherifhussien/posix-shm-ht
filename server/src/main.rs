#![allow(warnings)]

mod args;
mod hash_table;
mod ipc;
mod shmem;
mod sem;

use std::thread;
use std::sync::{Arc, Mutex};

use tokio::signal;
use env_logger::Env;
use log::{info, warn};

use ipc::IPC;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Server ***********************");

    // parse cli args
    let ht_size: usize = args::parse_args();
    
    let mut ipc = match IPC::init(ht_size) {
        Ok(ipc) => {
            info!("IPC >> server initialized successfully and ready for requests!");
            Arc::new(Mutex::new(ipc))
        },
        Err(err) => {
            warn!("IPC >> init error: {}", err);
            let my_error = std::io::Error::new(std::io::ErrorKind::Other, "custom error");
            return Err(Box::new(my_error) as Box<dyn std::error::Error>)
        },
    };

    // waits for requests and loop
    
    let ipc_clone_1 = Arc::clone(&ipc);
    thread::spawn(move || {
        ipc_clone_1.lock().unwrap().req_handler();
    });

    let ipc_clone_2 = Arc::clone(&ipc);
    thread::spawn(move || {

        match ipc_clone_2.lock().unwrap().clean() {
            Ok(_) => info!("IPC >> cleaned successfully"),
            Err(err) => warn!("IPC >> clean error: {}", err),
        }
    });

    signal::ctrl_c().await?;
    println!("ctrl-c received!");

    Ok(())
    
}