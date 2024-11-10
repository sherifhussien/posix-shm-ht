use std::os::raw::c_void;

use log::{info, warn};
use libc::{sem_t, sem_open, sem_close, sem_wait, sem_post, sem_unlink};


use utils::message::Message;
use utils::shared_mem::{SharedMemory, Q_CAPACITY};


// to remove
use std::{thread, time::{self, Duration}};

// TODO: handle race conditions
// TODO: Handle full queue
/// enqueue to requests buffer
pub fn enqueue(shm_ptr: *mut c_void, sem:*mut sem_t, message: Message) {
    // raw pointer
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm = unsafe{ &mut *shm_ptr };

    unsafe {
        info!("trying to aquire lock");
        let aquired = sem_wait(sem) == 0;
        info!("aquired lock {aquired}");
    };

    const DURATION: Duration = time::Duration::from_secs(30);
    thread::sleep(DURATION);

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

    unsafe {
        let released = sem_post(sem) == 0;
        info!("lock released: {released}");
    };

}

// TODO: handle race conditions
// TODO: Handle empty queue
/// dequeue from response buffer
pub fn dequeue(shm_ptr: *mut c_void, sem:*mut sem_t) -> Message {
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm = unsafe { &mut *shm_ptr };

    unsafe {
        sem_wait(sem);
    };

    /* enters critical region */

    // gets front value
    let front = shm.res_front;

    // gets reponse buffer
    let buffer = &mut shm.res_buffer;

    let message = buffer[front].clone();

    // writes front value
    shm.res_front = (shm.res_front + 1) % Q_CAPACITY;

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

    let buffer = &shm.res_buffer;

    info!("inside shm_read m1 {:?}", buffer[0]);
    info!("inside shm_read m2 {:?}", buffer[1]);
    info!("inside shm_read m3 {:?}", buffer[2]);
    
    buffer[0].clone()
}