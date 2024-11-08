mod args;
mod hash_table;
mod ipc;

use log::{info, warn};
use env_logger::Env;

use ipc::SharedMemory;
use hash_table::HashTable;


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let ht_size: i32 = args::parse_args();
    info!("Hash table size >> {ht_size}");

    let mut ht: HashTable<String, i32> = hash_table::HashTable::new(ht_size as usize);
    info!("{:?}", ht);

    let mut shm: SharedMemory = SharedMemory::new();
    
    match shm.init() {
        Ok(_) => info!("ipc >> created and mapped shm"),
        Err(err) => warn!("ipc >> creation error {}", err),
    }
    
    let m = shm.read();
    info!("Read from shared memory >> {:?}", m);

    shm.write(10);

    let m = shm.read();
    info!("Read from shared memory >> {:?}", m);

    match shm.clean() {
        Ok(_) => info!("ipc >> cleaned shm"),
        Err(err) => warn!("ipc >> cleaning error {}", err),
    }
}