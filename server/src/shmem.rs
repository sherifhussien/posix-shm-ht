use std::io::{self, Error, ErrorKind};
use std::os::raw::c_void;

use log::{info, warn};
use libc::sem_t;

use utils::message::Message;
use utils::shared_mem::{SharedMemory, Q_CAPACITY};
use utils::sem;

/// enqueue to response buffer
pub fn enqueue(shm: &mut SharedMemory, mutex: *mut sem_t, sig: *mut sem_t, message: Message) -> io::Result<()> {

    sem::wait(mutex)?;

    /* enters critical region */
    
    let size = shm.res_size;
    if size == Q_CAPACITY {
        // release lock 
        sem::post(mutex)?;
        return Err(Error::new(ErrorKind::OutOfMemory, "enqueue >> response buffer is full"));
    }

    // gets rear value
    let rear = shm.res_rear;

    // gets responses buffer
    let buffer = &mut shm.res_buffer;

    // writes message into buffer
    buffer[rear] = message;

    // writes rear value
    shm.res_rear = (shm.res_rear + 1) % Q_CAPACITY;

    // writes size
    shm.res_size = shm.res_size + 1;

    /* leaves critical region */

    sem::post(mutex)?;

    //signal client that response was enqueued
    sem::post(sig)?;
    // info!(">> signal sent to client");

    Ok(())
}

/// dequeue from requests buffer
pub fn dequeue(shm: &mut SharedMemory, mutex: *mut sem_t) -> io::Result<Message> {

    sem::wait(mutex)?;

    /* enters critical region */

    let size = shm.req_size;
    if size == 0 {
        // release lock 
        sem::post(mutex)?;
        return Err(Error::new(ErrorKind::NotFound, "dequeue >> requests buffer is empty"));
    }

    // gets front value
    let front = shm.req_front;

    // gets requests buffer
    let buffer = &mut shm.req_buffer;

    let message = buffer[front].clone();

    // clear buffer entry
    buffer[front] = Message::empty();

    // writes front value
    shm.req_front = (shm.req_front + 1) % Q_CAPACITY;

    // writes size
    shm.req_size = shm.req_size - 1;

    /* leaves critical region */

    sem::post(mutex)?;

    Ok(message)
}