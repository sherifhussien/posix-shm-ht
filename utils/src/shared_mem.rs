use std::mem::size_of;

use crate::message::Message;

pub const SHM_NAME: &str = "/my-shm";
pub const SHM_SIZE: usize = size_of::<SharedMemory>();
pub const Q_CAPACITY: usize = 10;

#[repr(C)]
pub struct SharedMemory {
    // request
    pub req_buffer: [Message; Q_CAPACITY],
    pub req_front: usize,
    pub req_rear: usize,
    pub req_size: usize,

    // response
    pub res_buffer: [Message; Q_CAPACITY],
    pub res_front: usize,
    pub res_rear: usize,
    pub res_size: usize,
}