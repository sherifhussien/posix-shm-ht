mod ipc;
mod shared_mem;

use std::{thread, time::{self, Duration}};

use log::{info, warn};
use env_logger::Env;

use ipc::IPC;

const DURATION: Duration = time::Duration::from_secs(5);

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut shm: IPC = IPC::new();
    
    match shm.init() {
        Ok(_) => info!("ipc >> mapped shm"),
        Err(err) => warn!("ipc >> creation error {}", err),
    }

    let m = shm.read();
    info!("Read from shared memory >> {:?}", m);

    thread::sleep(DURATION);
    thread::sleep(DURATION);
    thread::sleep(DURATION);

    let m = shm.read();
    info!("Read from shared memory >> {:?}", m);

    match shm.clean() {
        Ok(_) => info!("ipc >> cleaned shm"),
        Err(err) => warn!("ipc >> cleaning error {}", err),
    }
}
