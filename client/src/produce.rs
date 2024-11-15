use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;
use libc::sem_t;
use log::{info, warn};

use crate::ipc::IPC;
use utils::message::{Message, MessageType, VALUE_SIZE};
use utils::shared_mem::SharedMemory;

pub fn produce_message(shm: &mut SharedMemory, req_mutex: *mut sem_t, s_sig: *mut sem_t) {
    let base_key = "key";
    let base_value = "value";
    let msg_types: [MessageType; 3] = [MessageType::Get, MessageType::Insert, MessageType::Remove];

    let start = Instant::now();
    
    for i in 1..500000 {

        let random_type = rand::thread_rng().gen_range(0..=2);
        let random_key_index = rand::thread_rng().gen_range(1..=50);
        let key = format!("{}{}", base_key, random_key_index);
        let random_value_index = rand::thread_rng().gen_range(1..=1000);
        let value = format!("{}{}", base_value, random_value_index);

        let message = match msg_types[random_type] {
            MessageType::Get => {
                Message {
                    typ: MessageType::Get,
                    key: Message::serliaize_key(&key),
                    value: [0; VALUE_SIZE]
                }
            },
            MessageType::Remove => {
                Message {
                    typ: MessageType::Remove,
                    key: Message::serliaize_key(&key),
                    value: [0; VALUE_SIZE]
                }
            },
            _ => {
                Message {
                    typ: MessageType::Insert,
                    key: Message::serliaize_key(&key),
                    value: Message::serliaize_value(&value),
                }
            }
        };

        IPC::write(shm, req_mutex, s_sig, message);
    }

    let duration = start.elapsed();
    thread::sleep(Duration::from_secs(2));
    info!(">> Time taken: {:?}", duration);
}