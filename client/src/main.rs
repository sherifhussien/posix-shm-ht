mod ipc;
mod shmem;

use std::{thread, time::{self, Duration}};

use log::{info, warn};
use env_logger::Env;

use utils::message::{self, Message};
use ipc::IPC;

const DURATION: Duration = time::Duration::from_secs(10);

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("***** Started Client *****");

    let mut ipc: IPC = IPC::new();
    
    match ipc.init() {
        Ok(_) => info!("ipc >> mapped shm"),
        Err(err) => warn!("ipc >> creation error {}", err),
    }
    
    /* test */

    let message = Message {
        typ: message::CLIENT_GET,
        content: string_to_fixed_array("test")
    };
    ipc.write(message);
    
    let message = Message {
        typ: message::CLIENT_GET,
        content: string_to_fixed_array("tesu")
    };
    ipc.write(message);
    
    let message = Message {
        typ: message::CLIENT_GET,
        content: string_to_fixed_array("tesv")
    };
    ipc.write(message);

    thread::sleep(DURATION);

    info!("should consume 3 reponses");
    ipc.debug_read();

    /* test */
        
    match ipc.clean() {
        Ok(_) => info!("ipc >> cleaned shm"),
        Err(err) => warn!("ipc >> cleaning error {}", err),
    }
}

fn string_to_fixed_array(s: &str) -> [u8; 16] {
    let mut array = [0u8; 16];
    let bytes = s.as_bytes();

    let len = bytes.len().min(16);
    array[..len].copy_from_slice(&bytes[..len]);

    array
}
