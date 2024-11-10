use std::os::raw::c_void;

use log::{info, warn};
use libc::{sem_t, sem_open, sem_close, sem_wait, sem_post, sem_unlink, O_RDWR};



use utils::message::{self, Message};
use utils::shared_mem::{SharedMemory, Q_CAPACITY};

// TODO: handle race conditions
// TODO: Handle full queue
/// enqueue to response buffer
pub fn enqueue(shm_ptr: *mut c_void, sem:*mut sem_t, message: Message) {
    // raw pointer
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm = unsafe{ &mut *shm_ptr };

    unsafe {
        sem_wait(sem);
    };

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

    unsafe {
        sem_post(sem);
    };
}

// TODO: handle race conditions
// TODO: Handle empty queue
/// dequeue from requests buffer
pub fn dequeue(shm_ptr: *mut c_void, sem:*mut sem_t) -> Message {
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm = unsafe { &mut *shm_ptr };

    unsafe {
        sem_wait(sem);
    };

    /* enters critical region */

    // gets front value
    let front = shm.req_front;

    // gets requests buffer
    let buffer = &mut shm.req_buffer;

    let message = buffer[front].clone();

    // writes front value
    shm.req_front = (shm.req_front + 1) % Q_CAPACITY;

    /* leaves critical region */

    unsafe {
        sem_post(sem);
    };

    message
}


/// read from shm - used for debugging
pub fn shm_read(shm_ptr: *mut c_void, sem:*mut sem_t) -> Message {    
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm = unsafe { &*shm_ptr };

    unsafe {
        info!("trying to aquire lock");
        let aquired = sem_wait(sem) == 0;
        info!("aquired lock {aquired}");
    };

    let buffer = &shm.req_buffer;
    let message = buffer[0].clone();

    info!("inside shm_read m1 {:?}", buffer[0]);

    unsafe {
        let released = sem_post(sem) == 0;
        info!("lock released: {released}");
    };
    
    message
}