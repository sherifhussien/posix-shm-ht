mod args;
mod hash_table;
mod ipc;
mod shmem;
mod sem;

use std::{thread, time};

use log::{info, warn};
use env_logger::Env;

use utils::message::{self, Message};
use utils::serliaize;
use hash_table::HashTable;
use ipc::IPC;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("*********************** Started Server ***********************");

    // parse cli args
    let ht_size: i32 = args::parse_args();

    // construct ht
    let ht: HashTable<String, i32> = hash_table::HashTable::new(ht_size as usize);
    info!("A hash table with size {} was constructed", ht_size);

    let mut ipc: IPC = IPC::new();
    
    match ipc.init() {
        Ok(_) => info!("shared memory object was created and mapped successfully"),
        Err(err) => warn!("unable to create or map shared memory object: {}", err),
    }

    /* start tests */
    thread::sleep(time::Duration::from_secs(10));
    test_1(&ipc);
    /* end tests */

    match ipc.clean() {
        Ok(_) => info!("cleaned shared memory object"),
        Err(err) => warn!("unable to clean shared memory object: {}", err),
    }
}

fn test_1(ipc: &IPC) {
    ipc.debug_read();
    
    // let message = Message {
    //     typ: message::CLIENT_GET,
    //     content: serliaize("tesw")
    // };

    // match  ipc.write(message) {
    //     Ok(_) => info!("message was enqueued!"),
    //     Err(err) => warn!("message can't be enqueued: {}", err),
    // }
}