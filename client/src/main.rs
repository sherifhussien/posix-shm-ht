mod ipc;
mod shmem;
mod sem;

use std::{thread, time::{self, Duration}};

use log::{info, warn};
use env_logger::Env;

use utils::{serliaize, deserialize};
use utils::message::{self, Message};
use ipc::IPC;

const DURATION: Duration = time::Duration::from_secs(5);

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
        content: serliaize("test")
    };
    ipc.write(message);

    thread::sleep(DURATION);

    // info!("should consume 3 reponses");
    // ipc.debug_read();

    /* test */
        
    match ipc.clean() {
        Ok(_) => info!("ipc >> cleaned shm"),
        Err(err) => warn!("ipc >> cleaning error {}", err),
    }
}