use std::os::raw::c_void;
use std::ptr::null_mut;

use rustix::{io, shm};
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, munmap, ProtFlags, MapFlags};

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
    
    pub fn init(&mut self) -> io::Result<()>  {
        // create the shm
        let fd = shm::open(
            SHM_NAME,
            shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        )?;

        // resize the shm object, default is 0
        ftruncate(&fd, SHM_SIZE as u64)?;

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

    /// unmap and remove the shm object
    pub fn clean(&self) -> io::Result<()> {
        unsafe {
            munmap(self.ptr, SHM_SIZE)?;
        }

        shm::unlink(SHM_NAME)?;

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