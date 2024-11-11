use std::io;
use std::os::raw::c_void;
use std::ptr::null_mut;

use libc::sem_t;
use rustix::shm;
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, munmap, ProtFlags, MapFlags};
use log::{info, warn};


use crate::sem;
use crate::shmem;
use utils::message::Message;
use utils::shared_mem::{SHM_NAME, SHM_SIZE};
use utils::sem::{REQ_MUTEX_NAME, RES_MUTEX_NAME};

pub struct IPC {
    shm_ptr: *mut c_void,
    req_mutex: *mut sem_t,
    res_mutex: *mut sem_t,
}

impl IPC {

    pub fn new() -> Self {
        IPC {
            shm_ptr: null_mut(),
            req_mutex: null_mut(), // controls access to critical region
            res_mutex: null_mut(), // controls access to critical region
        }
    }
    
    pub fn init(&mut self) -> io::Result<()> {
        // create the sem objects
        let req_mutex: *mut sem_t = sem::open(REQ_MUTEX_NAME)?;
        info!(">> opened res_mutex semaphore with descriptor: {:?}", req_mutex);

        let res_mutex: *mut sem_t = sem::open(RES_MUTEX_NAME)?;
        info!(">> opened req_mutex semaphore with descriptor: {:?}", res_mutex);

        // create the shm
        let fd = shm::open(
            SHM_NAME,
            shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        )?;
        info!(">> opened shm with descriptor: {:?}", fd);

        // resize the shm object, default is 0
        ftruncate(&fd, SHM_SIZE as u64)?;
        info!(">> resized shm");

        // map the shm object into our address space
        let shm_ptr: *mut c_void = unsafe {
            mmap(
                null_mut(),
                SHM_SIZE,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                fd,
                0,
            )?
        };
        info!(">> mapped shm");

        self.shm_ptr = shm_ptr;
        self.req_mutex = req_mutex;
        self.res_mutex = res_mutex;

        Ok(())
    }

    /// unmap and remove the shm object
    pub fn clean(&self) -> io::Result<()> {
        
        sem::close(self.req_mutex)?;
        info!(">> closed req_mutex semaphore with descriptor: {:?}", self.req_mutex);
        sem::close(self.res_mutex)?;
        info!(">> closed res_mutex semaphore with descriptor: {:?}", self.res_mutex);

        sem::destroy(REQ_MUTEX_NAME)?;
        info!(">> removed req_mutex semaphore with name: {}", REQ_MUTEX_NAME);
        sem::destroy(RES_MUTEX_NAME)?;
        info!(">> removed res_mutex semaphore with name: {}", RES_MUTEX_NAME);
        
        unsafe {
            munmap(self.shm_ptr, SHM_SIZE)?;
            info!(">> unmapped shm");
        }

        shm::unlink(SHM_NAME)?;
        info!(">> removed shm with name: {}", SHM_NAME);

        Ok(())
    }

     // writes message to shm
     pub fn write(&self, message: Message) -> io::Result<()> {
        shmem::enqueue(self.shm_ptr, self.res_mutex, message.clone())?;
        info!(">> message enqueued code: {}", message.typ);

        Ok(())
    }

    // reads message from shm
    pub fn read(&self) -> io::Result<Message> {
        let message = shmem::dequeue(self.shm_ptr, self.req_mutex)?;
        info!(">> message dequeued code: {}", message.typ);

        Ok(message)
    }

    /// read from shm - used for debugging
    pub fn debug_read(&self) -> io::Result<Message> {
        let message = shmem::shm_read(self.shm_ptr, self.req_mutex)?;

        Ok(message)
    }
}