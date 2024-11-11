use std::ffi::CString;

use log::{info, warn};
use libc::{
    sem_t, sem_open, sem_close, __error, 
    SEM_FAILED, EEXIST, EACCES, EINVAL, 
    ENAMETOOLONG, ENOENT, ENOMEM,
};

// open the sem
pub fn open(sem_name: &str) -> *mut sem_t {
    let sem_name = CString::new(sem_name).unwrap();
    let sem: *mut sem_t = unsafe {
        sem_open(
            sem_name.as_ptr(),
            0,
        )
    };

    if sem == SEM_FAILED {
        let errno: i32 = unsafe { *__error() };
        match errno {
            EACCES => warn!("sem_open >> permission denied"),
            EEXIST => warn!("sem_open >> a semaphore with this name already exists"),
            EINVAL => warn!("sem_open >> invalid argument"),
            ENAMETOOLONG => warn!("sem_open >> name too long"),
            ENOENT => warn!("sem_open >> no such file or directory"),
            ENOMEM => warn!("sem_open >> insufficient memory"),
            _ => warn!("sem_open >> an unknown error occurred: errno = {}", errno)
        }
    } else {
        info!("opened semaphore with descriptor: {:?}", sem)
    }
    sem
}

pub fn close(sem: *mut sem_t) {
    unsafe {
        if sem_close(sem) == -1 {
            let errno: i32 = *__error();
            match errno {
                EINVAL => warn!("sem_close >> sem is not a valid semaphore"),
                _ => warn!("sem_close >> an unknown error occurred: errno = {}", errno)
            }
        }
    }
}