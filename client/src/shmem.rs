use std::ptr::write;
use std::os::raw::c_void;

use log::info;

use utils::message::Message;
use utils::shared_mem::{SharedMemory, Q_CAPACITY};

// TODO: handle race conditions
// TODO: Handle full queue
/// enqueue to requests buffer
pub fn enqueue(ptr: *mut c_void, message: Message) {
    // raw pointer
    let ptr = ptr as *mut SharedMemory;
    let shm = unsafe{ &mut *ptr };

    /* enters critical region */

    // gets rear value
    let rear = shm.req_rear;

    // gets requests buffer
    let buffer = &mut shm.req_buffer;

    // writes message into buffer
    buffer[rear] = message;

    // writes rear value
    shm.req_rear = (shm.req_rear + 1) % Q_CAPACITY;

    /* leaves critical region */
}

// TODO: handle race conditions
// TODO: Handle empty queue
/// dequeue from response buffer
pub fn dequeue(ptr: *mut c_void) -> Message {
    let ptr = ptr as *mut SharedMemory;
    let shm = unsafe { &mut *ptr };

    /* enters critical region */

    // gets front value
    let front = shm.res_front;

    // gets reponse buffer
    let buffer = &mut shm.res_buffer;

    let message = buffer[front].clone();

    // writes front value
    shm.res_front = (shm.res_front + 1) % Q_CAPACITY;

    /* leaves critical region */

    message
}

/// read from shm - used for debugging
pub fn shm_read(ptr: *mut c_void) -> Message {    
    let ptr = ptr as *mut SharedMemory;
    let shm = unsafe { &*ptr };

    let buffer = &shm.res_buffer;

    info!("inside shm_read m1 {:?}", buffer[0]);
    info!("inside shm_read m2 {:?}", buffer[1]);
    info!("inside shm_read m3 {:?}", buffer[2]);
    
    buffer[0].clone()
}

/// write to shm - used for debugging
pub fn shm_write<T>(ptr: *mut c_void, value: T ) {    
    unsafe {
        // overwites a memory location
        write(ptr as *mut T,  value );
    }
}