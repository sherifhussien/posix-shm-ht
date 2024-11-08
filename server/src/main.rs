mod args;
mod hash_table;
mod ipc;
mod shared_mem;

use std::{thread, time::{self, Duration}};

use log::{info, warn};
use env_logger::Env;

use ipc::IPC;
use hash_table::HashTable;

const DURATION: Duration = time::Duration::from_secs(1);


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let ht_size: i32 = args::parse_args();
    info!("Hash table size >> {ht_size}");

    let mut ht: HashTable<String, i32> = hash_table::HashTable::new(ht_size as usize);
    info!("{:?}", ht);

    let mut shm: IPC = IPC::new();
    
    match shm.init() {
        Ok(_) => info!("ipc >> created and mapped shm"),
        Err(err) => warn!("ipc >> creation error {}", err),
    }
    
    let m = shm.read();
    info!("Read from shared memory >> {:?}", m);
    
    thread::sleep(DURATION);

    shm.write(50);

    // let m = shm.read();
    // info!("Read from shared memory >> {:?}", m);

    thread::sleep(DURATION);

    match shm.clean() {
        Ok(_) => info!("ipc >> cleaned shm"),
        Err(err) => warn!("ipc >> cleaning error {}", err),
    }
}