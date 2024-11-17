#![allow(warnings)]
mod args;
mod ipc;
mod shmem;
mod sem;
mod gen;
mod handler;

use std::{sync::Arc, thread};

use log::{info, warn};
use env_logger::Env;
use tokio::signal;

use ipc::IPC;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Client ***********************");    

    let test_mode: bool = args::parse_args();
    info!(">> is test mode: {}", test_mode);

    let ipc: Arc<IPC>  = match IPC::init() {
        Ok(ipc) => {
            info!("IPC >> initialized successfully!");
            Arc::new(ipc)
        },
        Err(err) => {
            warn!("IPC >> init error: {}", err);
            return Ok(())
        },
    };

    let ipc_clone = Arc::clone(&ipc);
    thread::spawn(move || loop {
        handler::response_handler(&ipc_clone);
    });

    if test_mode {
        gen::generate_messages(&ipc);
    }

    let ipc_clone = Arc::clone(&ipc);
    thread::spawn(move || loop {
        handler::input_handler(&ipc_clone);
    });

    signal::ctrl_c().await?;
    info!(">> SIGINT received!");

    match ipc.clean() {
        Ok(_) => info!("IPC >> cleaned successfully"),
        Err(err) => warn!("IPC >> clean error: {}", err),
    };
    
    Ok(())
}