use std::io;
use std::thread;
use std::sync::Arc;

use log::{info, warn};

use crate::sem;
use crate::shmem;
use crate::ipc::IPC;
use utils::shared_mem::SharedMemory;
use utils::message::{Message, MessageType, KEY_SIZE, VALUE_SIZE};

pub fn request_handler(ipc: &Arc<IPC>) {
    // wait for message on request buffer
    match sem::wait(ipc.s_sig) {
        Ok(_) => (),
        Err(err) => {
            warn!("req_handler >> can't aquire lock: {}", err);
            return;
        },
    }

    let ipc_clone = Arc::clone(&ipc);
    thread::spawn( move || {
        match read(&ipc_clone) {
            Ok(message) => {
                info!("request_handler >> read message: {:?} {:?} {:?}", message.typ, Message::deserialize_key(message.key), Message::deserialize_value(message.value));

                // operate on ht
                let msg = match message.typ {
                    MessageType::Get => get_handler(&ipc_clone, message),
                    MessageType::Insert => insert_handler(&ipc_clone, message),
                    MessageType::Remove => remove_handler(&ipc_clone, message),
                    _ => Message::empty()
                };

                // write message
                write(&ipc_clone, msg);
            },
            Err(err) => warn!("request_handler >> error reading message: {}", err),
        }
    });
}

fn get_handler(ipc: &Arc<IPC>, msg: Message) -> Message {
    let k = Message::deserialize_key(msg.key);

    match ipc.ht.lock().unwrap().get(&k) {
        Some(value) => Message {
            typ: MessageType::GetSuccess,
            key: msg.key,
            value: Message::serliaize_value(&value.clone()),
        },
        None => Message {
            typ: MessageType::GetNotFound,
            key: [0; KEY_SIZE],
            value: [0; VALUE_SIZE],
        },
    }
}

fn insert_handler(ipc: &Arc<IPC>, msg: Message) -> Message {
    let k = Message::deserialize_key(msg.key);
    let v = Message::deserialize_value(msg.value);

    match ipc.ht.lock().unwrap().insert(k, v) {
        Some(v) => Message {
            typ: MessageType::InsertSuccess,
            key: msg.key,
            value: Message::serliaize_value(&v),
        },
        None => Message {
            typ: MessageType::InsertSuccess,
            key: [0; KEY_SIZE],
            value: [0; VALUE_SIZE],
        },
    }
}

fn remove_handler(ipc: &Arc<IPC>, msg: Message) -> Message {
    let k = Message::deserialize_key(msg.key);

    match ipc.ht.lock().unwrap().remove(&k) {
        Some(v) => Message {
            typ: MessageType::RemoveSuccess,
            key: msg.key,
            value: Message::serliaize_value(&v),
        },
        None => Message {
            typ: MessageType::RemoveNotFound,
            key: [0; KEY_SIZE],
            value: [0; VALUE_SIZE],
        },
    }
}

// writes message to shm
pub fn write(ipc: &Arc<IPC>, message: Message) -> io::Result<()> {
    let shm: &mut SharedMemory = unsafe { &mut *ipc.shm_ptr };
    shmem::enqueue(shm, ipc.res_mutex, ipc.c_sig, message.clone())?;
    Ok(())
}

// reads message from shm
pub fn read(ipc: &Arc<IPC>) -> io::Result<Message> {
    let shm: &mut SharedMemory = unsafe { &mut *ipc.shm_ptr };
    let message = shmem::dequeue(shm, ipc.req_mutex)?;
    Ok(message)
}

