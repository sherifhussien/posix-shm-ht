use std::os::raw::c_void;
use std::ptr::null_mut;

use rustix::{io, shm};
use rustix::fs::Mode;
use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};

use utils::shared_mem::{SHM_NAME, SHM_SIZE};
use utils::message::Message;
use crate::shmem;

pub struct IPC {
    ptr: *mut c_void,
}

impl IPC {

    pub fn new() -> Self {
        IPC {
            ptr: null_mut(),
        }
    }
    
    pub fn init(&mut self) -> io::Result<()> {
        // open the shm object
        let fd = shm::open(
            SHM_NAME,
            shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        )?;


        // map the shm object into our address space
        let ptr = unsafe {
            mmap(
                null_mut(),
                SHM_SIZE,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                fd,
                0,
            )?
        };

        self.ptr = ptr;

        Ok(())
    }

    /// unmap the shm object
    pub fn clean(&self) -> io::Result<()> {
        unsafe {
            munmap(self.ptr, SHM_SIZE)?;
        }

        Ok(())
    }

    // writes message to shm
    pub fn write(&self, message: Message) -> io::Result<()> {
        shmem::enqueue(self.ptr, message);

        Ok(())
    }

    // reads message from shm
    pub fn read(&self) -> io::Result<Message> {
        let message = shmem::dequeue(self.ptr);
        Ok(message)
    }

    /// read from shm - used for debugging
    pub fn debug_read(&self) -> Message {    
        shmem::shm_read(self.ptr)
    }
}
