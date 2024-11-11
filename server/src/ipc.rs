use std::os::raw::c_void;
use std::ptr::null_mut;

use libc::sem_t;
use rustix::{io, shm};
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, munmap, ProtFlags, MapFlags};

use utils::shared_mem::{SHM_NAME, SHM_SIZE};
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
    
    pub fn init(&mut self) -> io::Result<()>  {
        // create the sem object
        let req_sem: *mut sem_t = sem::create(SEM_NAME_REQ);
        let res_sem: *mut sem_t = sem::create(SEM_NAME_RES);

        // create the shm
        let fd = shm::open(
            SHM_NAME,
            shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        )?;

        // resize the shm object, default is 0
        ftruncate(&fd, SHM_SIZE as u64)?;

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

        self.shm_ptr = shm_ptr;
        self.req_sem = req_sem;
        self.res_sem = res_sem;

        Ok(())
    }

    /// unmap and remove the shm object
    pub fn clean(&self) -> io::Result<()> {
        
        sem::close(self.req_sem);
        sem::close(self.res_sem);
        sem::destroy(SEM_NAME_REQ);
        sem::destroy(SEM_NAME_RES);
        
        unsafe {
            munmap(self.shm_ptr, SHM_SIZE)?
        }
        shm::unlink(SHM_NAME)?;

        Ok(())
    }

     // writes message to shm
     pub fn write(&self, message: Message) -> io::Result<()> {
        shmem::enqueue(self.shm_ptr, self.res_sem, message);

        Ok(())
    }

    // reads message from shm
    pub fn read(&self) -> io::Result<Message> {
        let message = shmem::dequeue(self.shm_ptr, self.req_sem);
        Ok(message)
    }

    /// read from shm - used for debugging
    pub fn debug_read(&self) -> Message {
        shmem::shm_read(self.shm_ptr, self.req_sem)
    }
}