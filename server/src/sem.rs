use std::os::raw::c_uint;
use std::ffi::CString;

use log::{info, warn};
use libc::{
    sem_t, sem_open, sem_close, sem_unlink, __error,
    O_CREAT, O_EXCL, S_IROTH, S_IWOTH, S_IRUSR, S_IWUSR, 
    SEM_FAILED, EEXIST, EACCES, EINVAL, 
    ENAMETOOLONG, ENOENT, ENOMEM,
};

const PERMISSIONS: c_uint = (S_IRUSR | S_IWUSR | S_IROTH | S_IWOTH) as c_uint;

// create the sem
pub fn create(sem_name: &str) -> *mut sem_t {
    let sem_name = CString::new(sem_name).unwrap();
    let sem: *mut sem_t = unsafe {
        sem_open(
            sem_name.as_ptr(),
            O_CREAT | O_EXCL,
            PERMISSIONS,
            1 // initial value
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
        info!("created semaphore with descriptor: {:?}", sem)
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

// remove sem object
pub fn destroy(name: &str) {
    let sem_name = CString::new(name).unwrap();
    
    unsafe {
        if sem_unlink(sem_name.as_ptr()) == -1 {
            let errno: i32 = *__error();
            match errno {
                EACCES => warn!("sem_unlink >> permission denied"),
                ENAMETOOLONG => warn!("sem_unlink >> name too long"),
                ENOENT => warn!("sem_unlink >> no semaphore with the given name"),
                _ => warn!("sem_close >> an unknown error occurred: errno = {}", errno)
            }
        }
    }
}