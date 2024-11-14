use std::io;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::thread;
use std::sync::{Arc, Mutex};

use libc::sem_t;
use rustix::shm;
use rustix::fs::Mode;
use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};
use log::{info, warn};

use crate::sem;
use crate::shmem;
use utils::message::Message;
use utils::shared_mem::{ SharedMemory, SHM_NAME, SHM_SIZE};
use utils::sem::{REQ_MUTEX_NAME, RES_MUTEX_NAME, S_SIGNAL_NAME, C_SIGNAL_NAME};

pub struct IPC {
    pub shm_ptr: *mut SharedMemory,
    pub req_mutex: *mut sem_t, // controls access to critical region
    pub res_mutex: *mut sem_t, // controls access to critical region
    pub s_sig: *mut sem_t, // control signals to server
    pub c_sig: *mut sem_t, // control signals to client
}

impl IPC {

    pub fn init() -> io::Result<Self> {
        // open the sem objects
        let req_mutex: *mut sem_t = sem::open(REQ_MUTEX_NAME)?;
        info!(">> opened req_mutex semaphore with descriptor: {:?}", req_mutex);

        let res_mutex: *mut sem_t = sem::open(RES_MUTEX_NAME)?;
        info!(">> opened res_mutex semaphore with descriptor: {:?}", res_mutex);

        let s_sig: *mut sem_t = sem::open(S_SIGNAL_NAME)?;
        info!(">> opened s_sig semaphore with descriptor: {:?}", s_sig);

        let c_sig: *mut sem_t = sem::open(C_SIGNAL_NAME)?;
        info!(">> opened c_sig semaphore with descriptor: {:?}", c_sig);

        // open the shm object
        let fd = shm::open(
            SHM_NAME,
            shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        )?;
        info!(">> opened shm with descriptor: {:?}", fd);

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

        Ok(IPC {
            shm_ptr: shm_ptr as *mut SharedMemory,
            req_mutex: req_mutex,
            res_mutex: res_mutex,
            s_sig,
            c_sig,
        })
    }

    /// unmap the shm object
    pub fn clean(&self) -> io::Result<()> {
        sem::close(self.req_mutex)?;
        info!(">> closed req_mutex semaphore with descriptor: {:?}", self.req_mutex);
        sem::close(self.res_mutex)?;
        info!(">> closed res_mutex semaphore with descriptor: {:?}", self.res_mutex);
        sem::close(self.s_sig)?;
        info!(">> closed s_sig semaphore with descriptor: {:?}", self.s_sig);
        sem::close(self.c_sig)?;
        info!(">> closed c_sig semaphore with descriptor: {:?}", self.c_sig);
        
        unsafe {
            munmap(self.shm_ptr as *mut c_void, SHM_SIZE)?;
            info!(">> unmapped shm");
        }

        Ok(())
    }

    pub fn res_handler(&self) {
        loop {
            // wait for message on response buffer
            match sem::wait(self.c_sig) {
                Ok(_) => (),
                Err(err) => {
                    warn!("res_handler >> can't aquire lock: {}", err);
                    continue;
                },
            }
            
            let shm: &mut SharedMemory = unsafe { &mut *self.shm_ptr };
            let res_mutex: &mut i32 = unsafe { &mut *self.res_mutex }; 
            
            // start thread that consume the request
            thread::spawn(|| {
                match IPC::read(shm, res_mutex) {
                    Ok(message) => info!("res_handler >> read message: {:?} {:?} {:?}", message.typ, Message::deserialize_key(message.key), Message::deserialize_value(message.value)),
                    Err(err) => warn!("res_handler >> error reading message: {}", err),
                }
            });
        }
    }

    // writes message to shm
    pub fn write(shm: &mut SharedMemory, req_mutex: *mut sem_t, s_sig: *mut sem_t, message: Message) -> io::Result<()> {
        shmem::enqueue(shm, req_mutex, s_sig, message.clone())?;
        Ok(())
    }

    // reads message from shm
    pub fn read(shm: &mut SharedMemory, res_mutex: *mut sem_t) -> io::Result<Message> {
        let message = shmem::dequeue(shm, res_mutex)?;
        Ok(message)
    }
}
