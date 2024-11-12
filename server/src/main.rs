mod args;
mod hash_table;
mod ipc;
mod shmem;
mod sem;

use log::{info, warn};
use env_logger::Env;

use ipc::IPC;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Server ***********************");

    // parse cli args
    let ht_size: usize = args::parse_args();

    let mut ipc: IPC = IPC::new(ht_size);
    
    match ipc.init() {
        Ok(_) => info!("IPC >> initialized successfully"),
        Err(err) => warn!("IPC >> init error: {}", err),
    }

    // loops and waits for requests
    ipc.req_handler();

    match ipc.clean() {
        Ok(_) => info!("IPC >> cleaned successfully"),
        Err(err) => warn!("IPC >> clean error: {}", err),
    }
}