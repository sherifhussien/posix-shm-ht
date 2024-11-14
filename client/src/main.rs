#![allow(warnings)]

mod ipc;
mod shmem;
mod sem;
mod cli;

use libc::sem_t;
use log::{info, warn};
use env_logger::Env;

use ipc::IPC;
use utils::shared_mem::SharedMemory;

use std::thread;
use std::time::Duration;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Client ***********************");    

    let mut ipc: IPC = match IPC::init() {
        Ok(ipc) => {
            info!("IPC >> client initialized successfully!");
            ipc
        },
        Err(err) => {
            warn!("IPC >> init error: {}", err);
            return;
        },
    };

    // TODO: handle logic to either run script or read from cli
    // reads message from cli and loop
    let shm: &mut SharedMemory = unsafe { &mut *ipc.shm_ptr };
    let res_mutex: &mut sem_t = unsafe { &mut *ipc.res_mutex }; 
    let s_sig: &mut sem_t = unsafe { &mut *ipc.s_sig };
    thread::spawn(|| {
        cli::read(shm, res_mutex, s_sig);
    });

    ipc.res_handler();
  
    match ipc.clean() {
        Ok(_) => info!("IPC >> cleaned successfully"),
        Err(err) => warn!("IPC >> clean error: {}", err),
    }
}