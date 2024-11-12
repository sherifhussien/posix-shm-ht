use std::io::{self, Error, ErrorKind};
use std::os::raw::c_void;

use log::{info, warn};
use libc::sem_t;

use utils::message::Message;
use utils::shared_mem::{SharedMemory, Q_CAPACITY};
use utils::sem;

/// enqueue to requests buffer
pub fn enqueue(shm_ptr: *mut c_void, sem:*mut sem_t, sig:*mut sem_t, message: Message) -> io::Result<()> {
    // raw pointer
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm: &mut SharedMemory = unsafe{ &mut *shm_ptr };

    sem::wait(sem)?;

    /* enters critical region */

    let size = shm.req_size;
    if size == Q_CAPACITY {
        // release lock 
        sem::post(sem)?;
        return Err(Error::new(ErrorKind::OutOfMemory, "enqueue >> requests buffer is full"));
    }

    // gets rear value
    let rear = shm.req_rear;

    // gets requests buffer
    let buffer = &mut shm.req_buffer;

    // writes message into buffer
    buffer[rear] = message;

    // writes rear value
    shm.req_rear = (shm.req_rear + 1) % Q_CAPACITY;

    // writes size
    shm.req_size = shm.req_size + 1;

    /* leaves critical region */

    sem::post(sem)?;

    //signal server that request was enqueued
    sem::post(sig)?;
    info!(">> signal sent to server");

    Ok(())
}

/// dequeue from response buffer
pub fn dequeue(shm_ptr: *mut c_void, sem:*mut sem_t) -> io::Result<Message> {
    let shm_ptr = shm_ptr as *mut SharedMemory;
    let shm: &mut SharedMemory = unsafe { &mut *shm_ptr };

    sem::wait(sem)?;

    /* enters critical region */

    let size = shm.res_size;
    if size == 0 {
        // release lock 
        sem::post(sem)?;
        return Err(Error::new(ErrorKind::NotFound, "dequeue >> response buffer is empty"));
    }

    // gets front value
    let front = shm.res_front;

    // gets reponse buffer
    let buffer = &mut shm.res_buffer;

    let message = buffer[front].clone();

    // writes front value
    shm.res_front = (shm.res_front + 1) % Q_CAPACITY;

    // writes size
    shm.res_size = shm.res_size - 1;

    /* leaves critical region */

    sem::post(sem)?;

    Ok(message)
}