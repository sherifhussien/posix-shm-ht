use std::io;
use std::os::raw::c_void;
use std::ptr::null_mut;

use libc::sem_t;
use rustix::shm;
use rustix::fs::Mode;
use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};
use log::{info, warn};

use crate::sem;
use crate::shmem;
use utils::message::Message;
use utils::shared_mem::{ SHM_NAME, SHM_SIZE};
use utils::sem::{REQ_MUTEX_NAME, RES_MUTEX_NAME, S_SIGNAL_NAME, C_SIGNAL_NAME};

pub struct IPC {
    shm_ptr: *mut c_void,
    req_mutex: *mut sem_t, // controls access to critical region
    res_mutex: *mut sem_t, // controls access to critical region
    s_sig: *mut sem_t, // control signals to server
    c_sig: *mut sem_t, // control signals to client
}

impl IPC {

    pub fn new() -> Self {
        IPC {
            shm_ptr: null_mut(),
            req_mutex: null_mut(),
            res_mutex: null_mut(),
            s_sig: null_mut(),
            c_sig: null_mut(),
        }
    }
    
    pub fn init(&mut self) -> io::Result<()> {
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

        self.shm_ptr = shm_ptr;
        self.req_mutex = req_mutex;
        self.res_mutex = res_mutex;
        self.s_sig = s_sig;
        self.c_sig = c_sig;

        Ok(())
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
            munmap(self.shm_ptr, SHM_SIZE)?;
            info!(">> unmapped shm");
        }

        Ok(())
    }

    // writes message to shm
    pub fn write(&self, message: Message) -> io::Result<()> {
        shmem::enqueue(self.shm_ptr, self.req_mutex,self.s_sig, message.clone())?;
        info!(">> message enqueued");

        Ok(())
    }

    // reads message from shm
    pub fn read(&self) -> io::Result<Message> {
        let message = shmem::dequeue(self.shm_ptr, self.res_mutex)?;
        info!(">> message dequeued");

        Ok(message)
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

            // TODO: start thread that consume requests
            match self.read() {
                Ok(_) => info!(">> read message"),
                Err(err) => warn!(">> can't read message: {}", err),
            }

        }
    }
}
