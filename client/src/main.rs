mod ipc;
mod shmem;
mod sem;
mod cli;

use log::{info, warn};
use env_logger::Env;

use ipc::IPC;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Client ***********************");    
    
    let mut ipc: IPC = IPC::new();
    
     match ipc.init() {
        Ok(_) => info!("IPC >> initialized successfully"),
        Err(err) => warn!("IPC >> init error: {}", err),
    }

    // TODO: threads
    // ipc.res_handler();

    // TODO: handle logic to either run script or read from cli
    // loops and reads messages from cli
    cli::read(&ipc);
  
    match ipc.clean() {
        Ok(_) => info!("IPC >> cleaned successfully"),
        Err(err) => warn!("IPC >> clean error: {}", err),
    }
}