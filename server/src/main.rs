mod args;
mod hash_table;
mod ipc;
mod shmem;

use std::{thread, time::{self, Duration}};

use log::{info, warn};
use env_logger::Env;

use utils::message::{self, Message};
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
    
    info!("should consume 3 requests");
    ipc.debug_read();
    
    let message = Message {
        typ: message::CLIENT_GET,
        content: string_to_fixed_array("tesw")
    };
    ipc.write(message);
    
    let message = Message {
        typ: message::CLIENT_GET,
        content: string_to_fixed_array("tesx")
    };
    ipc.write(message);
    
    let message = Message {
        typ: message::CLIENT_GET,
        content: string_to_fixed_array("tesy")
    };
    ipc.write(message);

    thread::sleep(DURATION);
    thread::sleep(DURATION);

    /* test */

    match ipc.clean() {
        Ok(_) => info!("ipc >> cleaned shm"),
        Err(err) => warn!("ipc >> cleaning error >> {}", err),
    }
}

fn string_to_fixed_array(s: &str) -> [u8; 16] {
    let mut array = [0u8; 16];
    let bytes = s.as_bytes();

    let len = bytes.len().min(16);
    array[..len].copy_from_slice(&bytes[..len]);

    array
}