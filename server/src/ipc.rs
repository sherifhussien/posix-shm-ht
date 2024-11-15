use std::io;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};

use libc::sem_t;
use rustix::shm;
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, munmap, ProtFlags, MapFlags};
use log::info;


use crate::sem;
use crate::hash_table::HashTable;
use utils::shared_mem::{SharedMemory, SHM_NAME, SHM_SIZE};
use utils::sem::{REQ_MUTEX_NAME, RES_MUTEX_NAME, S_SIGNAL_NAME, C_SIGNAL_NAME};

pub struct IPC {
    pub shm_ptr: *mut SharedMemory,
    pub req_mutex: *mut sem_t,
    pub res_mutex: *mut sem_t,
    pub s_sig: *mut sem_t,
    pub c_sig: *mut sem_t,
    pub ht: Arc<Mutex<HashTable<String, String>>>,
}

// raw pointer are not thread safe, should guarantee safety manually
unsafe impl Send for IPC {}
unsafe impl Sync for IPC {}

impl IPC {

    pub fn init(ht_size: usize) -> io::Result<Self> {

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
        // TODO: in use by another thread
        // sem::close(self.s_sig)?;
        // info!(">> closed s_sig semaphore with descriptor: {:?}", self.s_sig);
        sem::close(self.c_sig)?;
        info!(">> closed c_sig semaphore with descriptor: {:?}", self.c_sig);

        sem::destroy(REQ_MUTEX_NAME)?;
        info!(">> removed req_mutex semaphore with name: {}", REQ_MUTEX_NAME);
        sem::destroy(RES_MUTEX_NAME)?;
        info!(">> removed res_mutex semaphore with name: {}", RES_MUTEX_NAME);
        sem::destroy(S_SIGNAL_NAME)?;
        info!(">> removed s_sig semaphore with name: {}", S_SIGNAL_NAME);
        sem::destroy(C_SIGNAL_NAME)?;
        info!(">> removed c_sig semaphore with name: {}", C_SIGNAL_NAME);
        
        unsafe {
            munmap(self.shm_ptr as *mut c_void, SHM_SIZE)?;
            info!(">> unmapped shm");
        }

        shm::unlink(SHM_NAME)?;
        info!(">> removed shm with name: {}", SHM_NAME);

        Ok(())
    }
}