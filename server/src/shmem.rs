use std::os::raw::c_void;

use log::info;

use utils::message::Message;
use utils::shared_mem::{SharedMemory, Q_CAPACITY};

// TODO: handle race conditions
// TODO: Handle full queue
/// enqueue to response buffer
pub fn enqueue(ptr: *mut c_void, message: Message) {
    // raw pointer
    let ptr = ptr as *mut SharedMemory;
    let shm = unsafe{ &mut *ptr };

    /* enters critical region */

    // gets rear value
    let rear = shm.res_rear;

    // gets responses buffer
    let buffer = &mut shm.res_buffer;

    // writes message into buffer
    buffer[rear] = message;

    // writes rear value
    shm.res_rear = (shm.res_rear + 1) % Q_CAPACITY;

    /* leaves critical region */
}

// TODO: handle race conditions
// TODO: Handle empty queue
/// dequeue from requests buffer
pub fn dequeue(ptr: *mut c_void) -> Message {
    let ptr = ptr as *mut SharedMemory;
    let shm = unsafe { &mut *ptr };

    /* enters critical region */

    // gets front value
    let front = shm.req_front;

    // gets requests buffer
    let buffer = &mut shm.req_buffer;

    let message = buffer[front].clone();

    // writes front value
    shm.req_front = (shm.req_front + 1) % Q_CAPACITY;

    /* leaves critical region */

    message
}


/// read from shm - used for debugging
pub fn shm_read(ptr: *mut c_void) -> Message {    
    let ptr = ptr as *mut SharedMemory;
    let shm = unsafe { &*ptr };

    let buffer = &shm.req_buffer;

    info!("inside shm_read m1 {:?}", buffer[0]);
    info!("inside shm_read m2 {:?}", buffer[1]);
    info!("inside shm_read m3 {:?}", buffer[2]);
    
    buffer[0].clone()
}