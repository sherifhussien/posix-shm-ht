mod args;
mod hash_table;
mod ipc;
mod shmem;
mod sem;

use std::{thread, time::{self, Duration}};

use log::{info, warn};
use env_logger::Env;

use utils::message::{self, Message};
use utils::serliaize;
use hash_table::HashTable;
use ipc::IPC;


const DURATION: Duration = time::Duration::from_secs(5);

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("***** Started Server *****");

    // let ht_size: i32 = args::parse_args();
    // info!("Hash table size >> {ht_size}");

    // let mut ht: HashTable<String, i32> = hash_table::HashTable::new(ht_size as usize);
    // info!("{:?}", ht);

    let mut ipc: IPC = IPC::new();
    
    match ipc.init() {
        Ok(_) => info!("ipc >> created and mapped shm"),
        Err(err) => warn!("ipc >> creation error >> {}", err),
    }

    /* test */
    
    thread::sleep(DURATION);
    
    info!("should consume 1 request");
    ipc.debug_read();
    
    // let message = Message {
    //     typ: message::CLIENT_GET,
    //     content: serliaize("tesw")
    // };
    // ipc.write(message);

    /* test */

    match ipc.clean() {
        Ok(_) => info!("ipc >> cleaned shm"),
        Err(err) => warn!("ipc >> cleaning error >> {}", err),
    }
}