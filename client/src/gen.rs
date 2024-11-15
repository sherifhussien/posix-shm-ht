use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;
use log::info;

use crate::ipc::IPC;
use crate::handler;
use utils::message::{Message, MessageType, VALUE_SIZE};

const N: i32 = 1000000;

// writes random messages
pub fn generate_messages(ipc: Arc<IPC>) {
    let base_key = "key";
    let base_value = "value";
    let msg_types: [MessageType; 3] = [MessageType::Get, MessageType::Insert, MessageType::Remove];

    let start = Instant::now();
    
    for _ in 1..N {
        let ipc_clone = Arc::clone(&ipc);

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

        handler::write(ipc_clone, message);
    }

    let duration = start.elapsed();
    thread::sleep(Duration::from_secs(2));
    info!(">> Time taken for {} messages: {:?}", N, duration);
}