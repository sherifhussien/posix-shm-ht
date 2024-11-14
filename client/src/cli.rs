use std::io::{stdin, stdout, Write};

use log::{info, warn};
use std::thread;
use libc::sem_t;

use crate::ipc::IPC;
use utils::message::{Message, MessageType, VALUE_SIZE};
use utils::shared_mem::SharedMemory;


pub fn read(shm: &mut SharedMemory, req_mutex: *mut sem_t, s_sig: *mut sem_t) {
    loop {
        println!(">> Enter a command: ");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let input: Vec<&str> = input.trim().split_whitespace().collect();

        if input.len() < 2 {
            println!(">> Invalid command!");
            continue;
        }

        // could use threads
        match input[0] {
            "get" => get_handler(shm, req_mutex, s_sig, input),
            "insert" => insert_handler(shm, req_mutex, s_sig, input),
            "remove" => remove_handler(shm, req_mutex, s_sig, input),
            _ => println!(">> Unknown command!"),
        }
    }
}

fn get_handler(shm: &mut SharedMemory, req_mutex: *mut sem_t, s_sig: *mut sem_t, input: Vec<&str>) {
    if input.len() != 2 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Get,
        key: Message::serliaize_key(input[1]),
        value: [0; VALUE_SIZE]
    };

    match IPC::write(shm, req_mutex, s_sig, message) {
        Ok(_) => info!("get_handler >> wrote message"),
        Err(err) => warn!("get_handler >> error writing message: {}", err),
    }
}

fn insert_handler(shm: &mut SharedMemory, req_mutex: *mut sem_t, s_sig: *mut sem_t, input: Vec<&str>) {
    if input.len() != 3 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Insert,
        key: Message::serliaize_key(input[1]),
        value: Message::serliaize_value(input[2]),
    };

    match IPC::write(shm, req_mutex, s_sig, message) {
        Ok(_) => info!("insert_handler >> wrote message"),
        Err(err) => warn!("insert_handler >> error writing message: {}", err),
    }
}

fn remove_handler(shm: &mut SharedMemory, req_mutex: *mut sem_t, s_sig: *mut sem_t, input: Vec<&str>) {
    if input.len() != 2 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Remove,
        key: Message::serliaize_key(input[1]),
        value: [0; VALUE_SIZE]
    };

    match IPC::write(shm, req_mutex, s_sig, message) {
        Ok(_) => info!("remove_handler >> wrote message"),
        Err(err) => warn!("remove_handler >> error writing message: {}", err),
    }
}