#![allow(warnings)]

mod args;
mod hash_table;
mod ipc;
mod shmem;
mod sem;

use env_logger::Env;
use log::{info, warn};

use ipc::IPC;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Server ***********************");

    // parse cli args
    let ht_size: usize = args::parse_args();
    
    let mut ipc: IPC = match IPC::init(ht_size) {
        Ok(ipc) => {
            info!("IPC >> server initialized successfully and ready for requests!");
            ipc
        },
        Err(err) => {
            warn!("IPC >> init error: {}", err);
            return;
        },
    };

    // waits for requests and loop
    ipc.req_handler();

    match ipc.clean() {
        Ok(_) => info!("IPC >> cleaned successfully"),
        Err(err) => warn!("IPC >> clean error: {}", err),
    }
}