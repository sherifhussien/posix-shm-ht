use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null_mut;

use rustix::{io, shm};
use rustix::fs::Mode;
use rustix::mm::{mmap, munmap, MapFlags, ProtFlags};

use crate::shared_mem::{self, SharedMemory};

const SHM_NAME: &str = "/my-shm";
const SHM_SIZE: usize = size_of::<SharedMemory>();


pub struct IPC {
    ptr: *mut c_void,
}

impl IPC {

    pub fn new() -> Self {
        IPC {
            ptr: null_mut(),
        }
    }
    
    /// create the shm object
    pub fn init(&mut self) -> io::Result<()>  {
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

    /// unmap the shm
    pub fn clean(&self) -> io::Result<()> {
        unsafe {
            munmap(self.ptr, SHM_SIZE)?;
        }

        Ok(())
    }

    /// read from shm
    pub fn read(&self) -> usize {
        shared_mem::shm_read(self.ptr)
    }

    /// write from shm
    pub fn write(&self, value: usize ) {
        shared_mem::shm_write(self.ptr, value)
    }
}
