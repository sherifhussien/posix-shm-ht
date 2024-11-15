#![allow(warnings)]
mod args;
mod hash_table;
mod ipc;
mod shmem;
mod sem;
mod handler;

use std::{sync::Arc, thread};

use log::{info, warn};
use env_logger::Env;
use tokio::signal;

use ipc::IPC;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    info!("*********************** Started Server ***********************");

    // parse cli args
    let ht_size: usize = args::parse_args();

    let ipc: Arc<IPC> = match IPC::init(ht_size) {
        Ok(ipc) => {
            info!("IPC >> server initialized successfully and ready for requests!");
            Arc::new(ipc)
        },
        Err(err) => {
            warn!("IPC >> init error: {}", err);
            return Ok(())
        },
    };

    // waits for requests and loop    
    let ipc_clone = Arc::clone(&ipc);
    thread::spawn(move || loop {
        let inner_clone = Arc::clone(&ipc_clone);
        handler::request_handler(inner_clone);
    });

    signal::ctrl_c().await?;
    println!("ctrl-c received!");

    match ipc.clean() {
        Ok(_) => info!("IPC >> cleaned successfully"),
        Err(err) => warn!("IPC >> clean error: {}", err),
    };

    Ok(())
    
}