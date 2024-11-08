use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null_mut;

use rustix::{io, shm};
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, munmap, ProtFlags, MapFlags};

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

    /// unmap and remove the shm
    pub fn clean(&self) -> io::Result<()> {
        unsafe {
            munmap(self.ptr, SHM_SIZE)?;
        }

        shm::unlink(SHM_NAME)?;

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