use std::io::{stdin, stdout, Write};

use log::{info, warn};

use crate::ipc::IPC;
use utils::message::{Message, MessageType, VALUE_SIZE};

pub fn read(ipc: &IPC) {
    loop {
        print!("Enter a command >> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let input: Vec<&str> = input.trim().split_whitespace().collect();

        if input.len() < 2 {
            println!(">> Invalid command!");
            continue;
        }

        // TODO: use threads
        match input[0] {
            "get" => get_handler(ipc, input),
            "insert" => insert_handler(ipc, input),
            "remove" => remove_handler(ipc, input),
            _ => println!(">> Unknown command!"),
        }
    }
}

fn get_handler(ipc: &IPC, input: Vec<&str>) {
    if input.len() != 2 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Get,
        key: Message::serliaize_key(input[1]),
        value: [0; VALUE_SIZE]
    };

    match ipc.write(message) {
        Ok(_) => info!(">> wrote message"),
        Err(err) => warn!(">> can't write message: {}", err),
    }
}

fn insert_handler(ipc: &IPC, input: Vec<&str>) {
    if input.len() != 3 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Insert,
        key: Message::serliaize_key(input[1]),
        value: Message::serliaize_value(input[2]),
    };

    match ipc.write(message) {
        Ok(_) => info!(">> wrote message"),
        Err(err) => warn!(">> can't write message: {}", err),
    }
}

fn remove_handler(ipc: &IPC, input: Vec<&str>) {
    if input.len() != 2 {
        println!("Invalid command!");
        return;
    }

    let message = Message {
        typ: MessageType::Remove,
        key: Message::serliaize_key(input[1]),
        value: [0; VALUE_SIZE]
    };

    match ipc.write(message) {
        Ok(_) => info!(">> wrote message"),
        Err(err) => warn!(">> can't write message: {}", err),
    }
}