use std::os::raw::{c_uint, c_void};
use std::ffi::CString;
use std::ptr::null_mut;

use libc::{sem_t, sem_open, sem_close, sem_wait, sem_post, sem_unlink, O_RDWR, O_CREAT, O_EXCL, SEM_FAILED, __error, S_IRUSR, S_IWUSR, EEXIST, EACCES, EINTR, EINVAL, EMFILE, ENAMETOOLONG, ENFILE, ENOENT, ENOMEM};
use rustix::{io, shm};
use rustix::fs::Mode;
use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};
use log::{info, warn};

use utils::shared_mem::{ SHM_NAME, SHM_SIZE};
use utils::message::Message;
use crate::shmem;
use crate::sem;

const SEM_NAME_REQ: &str = "/my-req-sem";
const SEM_NAME_RES: &str = "/my-res-sem";


pub struct IPC {
    shm_ptr: *mut c_void,
    req_sem: *mut sem_t,
    res_sem: *mut sem_t,
}

impl IPC {

    pub fn new() -> Self {
        IPC {
            shm_ptr: null_mut(),
            req_sem: null_mut(),
            res_sem: null_mut(),
        }
    }
    
    pub fn init(&mut self) -> io::Result<()> {
         // open the sem object
         let req_sem: *mut sem_t = sem::open(SEM_NAME_REQ);
         let res_sem: *mut sem_t = sem::open(SEM_NAME_RES);

        // open the shm object
        let fd = shm::open(
            SHM_NAME,
            shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        )?;


        // map the shm object into our address space
        let shm_ptr = unsafe {
            mmap(
                null_mut(),
                SHM_SIZE,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                fd,
                0,
            )?
        };

        self.shm_ptr = shm_ptr;
        self.req_sem = req_sem;
        self.res_sem = res_sem;

        Ok(())
    }

    /// unmap the shm object
    pub fn clean(&self) -> io::Result<()> {
        sem::close(self.req_sem);
        sem::close(self.res_sem);
        
        unsafe {
            munmap(self.shm_ptr, SHM_SIZE)?;
        }

        Ok(())
    }

    // writes message to shm
    pub fn write(&self, message: Message) -> io::Result<()> {
        shmem::enqueue(self.shm_ptr, self.req_sem, message);

        Ok(())
    }

    // reads message from shm
    pub fn read(&self) -> io::Result<Message> {
        let message = shmem::dequeue(self.shm_ptr, self.res_sem);
        Ok(message)
    }

    /// read from shm - used for debugging
    pub fn debug_read(&self) -> Message {    
        shmem::shm_read(self.shm_ptr, self.res_sem)
    }
}
