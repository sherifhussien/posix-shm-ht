use std::os::raw::c_void;
use std::{thread, time};

use log::{info, warn};
use libc::{sem_t, sem_wait, sem_post};

use utils::message::Message;
use utils::shared_mem::{SharedMemory, Q_CAPACITY};

// TODO: Handle full queue
/// enqueue to requests buffer
pub fn enqueue(shm_ptr: *mut c_void, sem:*mut sem_t, message: Message) {
    // raw pointer
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm: &mut SharedMemory = unsafe{ &mut *shm_ptr };

    info!("trying to aquire lock");
    let aquired = unsafe { sem_wait(sem) == 0 };
    info!("aquired lock: {aquired}");

    /* enters critical region */
    thread::sleep(time::Duration::from_secs(40));

    // gets rear value
    let rear = shm.req_rear;

    // gets requests buffer
    let buffer = &mut shm.req_buffer;

    // writes message into buffer
    buffer[rear] = message;

    // writes rear value
    shm.req_rear = (shm.req_rear + 1) % Q_CAPACITY;

    /* leaves critical region */

    let released = unsafe {sem_post(sem) == 0};
    info!("lock released: {released}");
}

// TODO: Handle empty queue
/// dequeue from response buffer
pub fn dequeue(shm_ptr: *mut c_void, sem:*mut sem_t) -> Message {
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm: &mut SharedMemory = unsafe { &mut *shm_ptr };

    info!("trying to aquire lock");
    let aquired = unsafe { sem_wait(sem) == 0 };
    info!("aquired lock: {aquired}");

    /* enters critical region */

    // gets front value
    let front = shm.res_front;

    // gets reponse buffer
    let buffer = &mut shm.res_buffer;

    let message = buffer[front].clone();

    // writes front value
    shm.res_front = (shm.res_front + 1) % Q_CAPACITY;

    /* leaves critical region */

    let released = unsafe {sem_post(sem) == 0};
    info!("lock released: {released}");

    message
}

/// read from shm - used for debugging
pub fn shm_read(shm_ptr: *mut c_void, sem:*mut sem_t) -> Message {    
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm: &SharedMemory = unsafe { &*shm_ptr };

    info!("trying to aquire lock");
    let aquired = unsafe { sem_wait(sem) == 0 };
    info!("aquired lock: {aquired}");

    let buffer = &shm.res_buffer;
    let message = buffer[0].clone();
    info!("inside shm_read message: {:?}", buffer[0]);

    let released = unsafe {sem_post(sem) == 0};
    info!("lock released: {released}");
    
    message
}