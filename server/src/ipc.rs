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
use crate::hash_table::HashTable;
use utils::message::Message;
use utils::shared_mem::{SHM_NAME, SHM_SIZE};
use utils::sem::{REQ_MUTEX_NAME, RES_MUTEX_NAME, S_SIGNAL_NAME, C_SIGNAL_NAME};

pub struct IPC {
    shm_ptr: *mut c_void,
    req_mutex: *mut sem_t, // controls access to critical region
    res_mutex: *mut sem_t, // controls access to critical region
    s_sig: *mut sem_t, // control signals to server
    c_sig: *mut sem_t, // control signals to client
    ht: HashTable<String, String>,
}

impl IPC {

    pub fn new(ht_size: usize) -> Self {
        IPC {
            shm_ptr: null_mut(), // segmentation error on null pointers
            req_mutex: null_mut(),
            res_mutex: null_mut(),
            s_sig: null_mut(),
            c_sig: null_mut(),
            ht: HashTable::new(ht_size as usize),
        }
    }
    
    pub fn init(&mut self) -> io::Result<()> {

        // TODO: Use SIGINT instead
        // workaround remove if they already exist
        sem::destroy(REQ_MUTEX_NAME);
        sem::destroy(RES_MUTEX_NAME);
        sem::destroy(S_SIGNAL_NAME);
        sem::destroy(C_SIGNAL_NAME);
        shm::unlink(SHM_NAME);


        // create the sem objects
        let req_mutex: *mut sem_t = sem::open(REQ_MUTEX_NAME, 1)?;
        info!(">> opened res_mutex semaphore with descriptor: {:?}", req_mutex);

        let res_mutex: *mut sem_t = sem::open(RES_MUTEX_NAME, 1)?;
        info!(">> opened req_mutex semaphore with descriptor: {:?}", res_mutex);

        let s_sig: *mut sem_t = sem::open(S_SIGNAL_NAME, 0)?;
        info!(">> opened s_sig semaphore with descriptor: {:?}", s_sig);

        let c_sig: *mut sem_t = sem::open(C_SIGNAL_NAME, 0)?;
        info!(">> opened c_sig semaphore with descriptor: {:?}", c_sig);

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
        self.s_sig = s_sig;
        self.c_sig = c_sig;

        Ok(())
    }

    /// unmap and remove the shm object
    pub fn clean(&self) -> io::Result<()> {
        
        sem::close(self.req_mutex)?;
        info!(">> closed req_mutex semaphore with descriptor: {:?}", self.req_mutex);
        sem::close(self.res_mutex)?;
        info!(">> closed res_mutex semaphore with descriptor: {:?}", self.res_mutex);
        sem::close(self.s_sig)?;
        info!(">> closed res_mutex semaphore with descriptor: {:?}", self.s_sig);
        sem::close(self.c_sig)?;
        info!(">> closed res_mutex semaphore with descriptor: {:?}", self.c_sig);

        sem::destroy(REQ_MUTEX_NAME)?;
        info!(">> removed req_mutex semaphore with name: {}", REQ_MUTEX_NAME);
        sem::destroy(RES_MUTEX_NAME)?;
        info!(">> removed res_mutex semaphore with name: {}", RES_MUTEX_NAME);
        sem::destroy(S_SIGNAL_NAME)?;
        info!(">> removed res_mutex semaphore with name: {}", S_SIGNAL_NAME);
        sem::destroy(C_SIGNAL_NAME)?;
        info!(">> removed res_mutex semaphore with name: {}", C_SIGNAL_NAME);
        
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
        shmem::enqueue(self.shm_ptr, self.res_mutex, self.c_sig, message.clone())?;
        info!(">> message enqueued code: {:?}", message.typ);

        Ok(())
    }

    // reads message from shm
    pub fn read(&self) -> io::Result<Message> {
        let message = shmem::dequeue(self.shm_ptr, self.req_mutex)?;
        info!(">> message dequeued code: {:?} {} {}", message.typ, Message::deserialize_key(message.key), Message::deserialize_value(message.value));

        Ok(message)
    }

    pub fn req_handler(&self) {
        loop {
            // wait for message on request buffer
            match sem::wait(self.s_sig) {
                Ok(_) => (),
                Err(err) => {
                    warn!("res_handler >> can't aquire lock: {}", err);
                    continue;
                },
            }

            // TODO: starts a thread that read request, operates on ht and then write
            match self.read() {
                Ok(message) => {
                    info!(">> read message");
                    // TODO: operate on ht (blocking)

                    // TODO: write message (blocking)
                },
                Err(err) => warn!(">> can't read message: {}", err),
            }
        }
    }
}