use std::io::{self, stdin};
use std::thread;
use std::sync::Arc;

use log::{info, warn};

use crate::sem;
use crate::shmem;
use crate::ipc::IPC;
use utils::shared_mem::SharedMemory;
use utils::message::{Message, MessageType, VALUE_SIZE};


pub fn input_handler(ipc: Arc<IPC>) {

    println!(">> Enter a command: ");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input: Vec<&str> = input.trim().split_whitespace().collect();

    if input.len() < 2 {
        println!(">> Invalid command!");
        return;
    }

    match input[0] {
        "get" => get_handler(ipc, input),
        "insert" => insert_handler(ipc, input),
        "remove" => remove_handler(ipc, input),
        _ => println!(">> Unknown command!"),
    }
}

fn get_handler(ipc: Arc<IPC>, input: Vec<&str>) {
    if input.len() != 2 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Get,
        key: Message::serliaize_key(input[1]),
        value: [0; VALUE_SIZE]
    };

    match write(ipc, message) {
        Ok(_) => info!("get_handler >> wrote message"),
        Err(err) => warn!("get_handler >> error writing message: {}", err),
    }
}

fn insert_handler(ipc: Arc<IPC>, input: Vec<&str>) {
    if input.len() != 3 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Insert,
        key: Message::serliaize_key(input[1]),
        value: Message::serliaize_value(input[2]),
    };

    match write(ipc, message) {
        Ok(_) => info!("insert_handler >> wrote message"),
        Err(err) => warn!("insert_handler >> error writing message: {}", err),
    }
}

fn remove_handler(ipc: Arc<IPC>, input: Vec<&str>) {
    if input.len() != 2 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Remove,
        key: Message::serliaize_key(input[1]),
        value: [0; VALUE_SIZE]
    };

    match write(ipc, message) {
        Ok(_) => info!("remove_handler >> wrote message"),
        Err(err) => warn!("remove_handler >> error writing message: {}", err),
    }
}

pub fn response_handler(ipc: Arc<IPC>) {
    // wait for message on response buffer
    match sem::wait(ipc.c_sig) {
        Ok(_) => (),
        Err(err) => {
            warn!("res_handler >> can't aquire lock: {}", err);
            return;
        },
    }

    // start thread that consume the message
    let ipc_clone = Arc::clone(&ipc);
    thread::spawn(|| {
        match read(ipc_clone) {
            Ok(message) => info!("response_handler >> read message: {:?} {:?} {:?}", message.typ, Message::deserialize_key(message.key), Message::deserialize_value(message.value)),
            Err(err) => warn!("response_handler >> error reading message: {}", err),
        }
    });
}

// writes message to shm
pub fn write(ipc: Arc<IPC>, message: Message) -> io::Result<()> {  
    let shm: &mut SharedMemory = unsafe { &mut *ipc.shm_ptr };
    shmem::enqueue(shm, ipc.req_mutex, ipc.s_sig, message.clone())?;
    Ok(())
}

// reads message from shm
pub fn read(ipc: Arc<IPC>) -> io::Result<Message> {
    let shm: &mut SharedMemory = unsafe { &mut *ipc.shm_ptr };
    let message = shmem::dequeue(shm, ipc.res_mutex)?;
    Ok(message)
}