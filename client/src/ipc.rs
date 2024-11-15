use std::io;
use std::os::raw::c_void;
use std::ptr::null_mut;

use libc::sem_t;
use rustix::shm;
use rustix::fs::Mode;
use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};
use log::info;

use crate::sem;
use utils::shared_mem::{SharedMemory, SHM_NAME, SHM_SIZE};
use utils::sem::{REQ_MUTEX_NAME, RES_MUTEX_NAME, S_SIGNAL_NAME, C_SIGNAL_NAME};

pub struct IPC {
    pub shm_ptr: *mut SharedMemory,
    pub req_mutex: *mut sem_t,
    pub res_mutex: *mut sem_t,
    pub s_sig: *mut sem_t,
    pub c_sig: *mut sem_t,
}

// raw pointer are not thread safe, should guarantee safety manually
unsafe impl Send for IPC {}
unsafe impl Sync for IPC {}


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
            req_mutex,
            res_mutex,
            s_sig,
            c_sig,
        })
    }

    /// unmap the shm object and close sem objects
    pub fn clean(&self) -> io::Result<()> {
        sem::close(self.req_mutex)?;
        info!(">> closed req_mutex semaphore with descriptor: {:?}", self.req_mutex);
        sem::close(self.res_mutex)?;
        info!(">> closed res_mutex semaphore with descriptor: {:?}", self.res_mutex);
        sem::close(self.s_sig)?;
        info!(">> closed s_sig semaphore with descriptor: {:?}", self.s_sig);
        // TODO: in use by another thread
        // sem::close(self.c_sig);
        // info!(">> closed c_sig semaphore with descriptor: {:?}", self.c_sig);
        
        unsafe {
            munmap(self.shm_ptr as *mut c_void, SHM_SIZE)?;
            info!(">> unmapped shm");
        }

        Ok(())
    }
}