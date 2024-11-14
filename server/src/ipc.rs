use std::io;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::thread;
use std::sync::{Arc, Mutex};

use libc::sem_t;
use rustix::shm;
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, munmap, ProtFlags, MapFlags};
use log::{info, warn};


use crate::sem;
use crate::shmem;
use crate::hash_table::HashTable;
use utils::message::{Message, MessageType, KEY_SIZE, VALUE_SIZE};
use utils::shared_mem::{SharedMemory, SHM_NAME, SHM_SIZE};
use utils::sem::{REQ_MUTEX_NAME, RES_MUTEX_NAME, S_SIGNAL_NAME, C_SIGNAL_NAME};

pub struct IPC {
    shm_ptr: *mut SharedMemory,
    req_mutex: *mut sem_t, // controls access to critical region
    res_mutex: *mut sem_t, // controls access to critical region
    s_sig: *mut sem_t, // control signals to server
    c_sig: *mut sem_t, // control signals to client
    ht: Arc<Mutex<HashTable<String, String>>>,
}

unsafe impl Send for IPC  {}


impl IPC {

    pub fn init(ht_size: usize) -> io::Result<Self> {

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

        Ok(IPC {
            shm_ptr: shm_ptr as *mut SharedMemory,
            req_mutex,
            res_mutex,
            s_sig,
            c_sig,
            ht: Arc::new(Mutex::new(HashTable::new(ht_size as usize))),
        })
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
            munmap(self.shm_ptr as *mut c_void, SHM_SIZE)?;
            info!(">> unmapped shm");
        }

        shm::unlink(SHM_NAME)?;
        info!(">> removed shm with name: {}", SHM_NAME);

        Ok(())
    }

    pub fn req_handler(&self) {
        loop {
            // wait for message on request buffer
            match sem::wait(self.s_sig) {
                Ok(_) => (),
                Err(err) => {
                    warn!("req_handler >> can't aquire lock: {}", err);
                    continue;
                },
            }

            let shm: &mut SharedMemory = unsafe { &mut *self.shm_ptr };
            let req_mutex: &mut sem_t = unsafe { &mut *self.req_mutex };
            let res_mutex: &mut sem_t = unsafe { &mut *self.res_mutex };
            let c_sig: &mut sem_t = unsafe { &mut *self.c_sig };
            let ht= Arc::clone(&self.ht);

            // TODO: starts a thread that read request, operates on ht and then write
            thread::spawn(move || {
                match IPC::read(shm, req_mutex) {
                    Ok(message) => {
                        info!("req_handler >> read message: {:?} {:?} {:?}", message.typ, Message::deserialize_key(message.key), Message::deserialize_value(message.value));

                        // TODO: operate on ht (blocking)
                        let m = match message.typ {
                            MessageType::Get => {
                                let k = Message::deserialize_key(message.key);

                                match ht.lock().unwrap().get(&k) {
                                    Some(value) => Message {
                                        typ: MessageType::GetSuccess,
                                        key: message.key,
                                        value: Message::serliaize_value(&value.clone()),
                                    },
                                    None => Message {
                                        typ: MessageType::GetNotFound,
                                        key: [0; KEY_SIZE],
                                        value: [0; VALUE_SIZE],
                                    },
                                }
                            },
                            MessageType::Insert => {
                                let k = Message::deserialize_key(message.key);
                                let v = Message::deserialize_value(message.value);

                                match ht.lock().unwrap().insert(k, v) {
                                    Some(v) => Message {
                                        typ: MessageType::InsertSuccess,
                                        key: message.key,
                                        value: Message::serliaize_value(&v),
                                    },
                                    None => Message {
                                        typ: MessageType::InsertSuccess,
                                        key: [0; KEY_SIZE],
                                        value: [0; VALUE_SIZE],
                                    },
                                }
                            },
                            MessageType::Remove => {
                                let k = Message::deserialize_key(message.key);

                                match ht.lock().unwrap().remove(&k) {
                                    Some(v) => Message {
                                        typ: MessageType::RemoveSuccess,
                                        key: message.key,
                                        value: Message::serliaize_value(&v),
                                    },
                                    None => Message {
                                        typ: MessageType::RemoveNotFound,
                                        key: [0; KEY_SIZE],
                                        value: [0; VALUE_SIZE],
                                    },
                                }
                            },
                            _ => Message::empty()
                        };

    
                        // write message (blocking)
                        match IPC::write(shm, res_mutex, c_sig, m) {
                            Ok(_) => info!("req_handler >> wrote message"),
                            Err(err) => warn!("req_handler >> error writing message: {}", err)
                        };
                    },
                    Err(err) => warn!("req_handler >> error reading message: {}", err),
                }
            });
        }
    }

    // writes message to shm
    pub fn write(shm: &mut SharedMemory, res_mutex: *mut sem_t, c_sig: *mut sem_t, message: Message) -> io::Result<()> {
        shmem::enqueue(shm, res_mutex, c_sig, message.clone())?;
        Ok(())
    }

    // reads message from shm
    pub fn read(shm: &mut SharedMemory, req_mutex: *mut sem_t) -> io::Result<Message> {
        let message = shmem::dequeue(shm, req_mutex)?;
        Ok(message)
    }
}