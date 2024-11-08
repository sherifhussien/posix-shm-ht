use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::{null_mut, write};

use rustix::fs::{ftruncate, Mode};
use rustix::mm::{self, mmap, munmap};
use rustix::{io, shm};

/// A type describing the data to be shared
#[repr(C)]
#[derive(Debug)] 
pub struct Message {
    value: i32,
}

const SHM_NAME: &str = "/my-shm";
const SHM_SIZE: usize = 4096; // 4KB

pub struct SharedMemory {
    ptr: Option<*mut c_void>, 
}

impl SharedMemory {

    pub fn new() -> Self {
        SharedMemory {
            ptr: None
        }
    }
    
    /// create or open the shm object
    pub fn init(&mut self) -> io::Result<()>  {
        let fd = shm::open(
            SHM_NAME,
            shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
            Mode::RUSR | Mode::WUSR,
        )?;

        // resize the shm object, default is 0
        ftruncate(&fd, SHM_SIZE as u64)?;

        // map the shared memory object into our address space
        let ptr = unsafe {
            mmap(
                null_mut(),
                SHM_SIZE,
                mm::ProtFlags::READ | mm::ProtFlags::WRITE,
                mm::MapFlags::SHARED,
                fd,
                0,
            )?
        };

        self.ptr = Some(ptr);

        Ok(())
    }

    /// unmap and remove the shm
    pub fn clean(&self) -> io::Result<()> {
        unsafe {
            munmap(self.ptr.unwrap(), size_of::<Message>())?;
        }

        shm::unlink(SHM_NAME)?;

        Ok(())
    }

    /// read from shm
    pub fn read(&self) -> &Message {
        let msg: &Message = unsafe { &*self.ptr.unwrap().cast::<Message>() };
        msg
    }

    /// write from shm
    pub fn write(&self, value:i32 ) {
        unsafe {
            let msg = self.ptr.unwrap().cast();
            write(msg, Message { value });
        }
    }
}