use std::os::raw::{c_uint, c_void};
use std::ffi::CString;

use log::{info, warn};
use libc::{sem_t, sem_open, sem_close, sem_wait, sem_post, sem_unlink, S_IROTH, S_IWOTH, O_RDWR, O_CREAT, O_EXCL, SEM_FAILED, __error, S_IRUSR, S_IWUSR, EEXIST, EACCES, EINTR, EINVAL, EMFILE, ENAMETOOLONG, ENFILE, ENOENT, ENOMEM};


const PERMISSIONS: c_uint = (S_IRUSR | S_IWUSR | S_IROTH | S_IWOTH) as c_uint;

// creaet the sem
pub fn create(sem_name: &str) -> *mut sem_t{
    let sem_name = CString::new(sem_name).unwrap();
    let sem: *mut sem_t = unsafe {
        sem_open(
            sem_name.as_ptr(),
            O_CREAT | O_EXCL,
            PERMISSIONS,
            1
        )
    };

    if sem == SEM_FAILED {
        unsafe {
            let errno = *__error();
            warn!("failed to create semaphore: errno {}", errno)
        }
    } else {
        info!("created sem with descriptor: {:?}", sem)
    }

    sem
}

pub fn close(sem: *mut sem_t) {
    unsafe {
        if sem_close(sem) == -1 {
            warn!("failed to close semaphore: error code {}", *__error());
        }
    }
}

// remove sem object
pub fn destroy(name: &str) {
    unsafe {
        let sem_name = CString::new(name).unwrap();
        if sem_unlink(sem_name.as_ptr()) == -1 {
            warn!("failed to remove semaphore: error code {}", *__error());
        }
    }
}