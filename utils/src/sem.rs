use std::io::{self, Error, ErrorKind};
use std::ffi::CString;
use std::os::raw::c_uint;

use log::{info, warn};
use libc::{
    sem_t, sem_open, sem_close, sem_wait, sem_post, sem_unlink, __error, 
    O_CREAT, O_EXCL, S_IROTH, S_IWOTH, S_IRUSR, S_IWUSR,
    SEM_FAILED, EEXIST, EACCES, EINVAL, ENAMETOOLONG, ENOENT, ENOMEM,
}; 

pub const REQ_MUTEX_NAME: &str = "/my-req-sem";
pub const RES_MUTEX_NAME: &str = "/my-res-sem";

const PERMISSIONS: c_uint = (S_IRUSR | S_IWUSR | S_IROTH | S_IWOTH) as c_uint;

pub enum AccessType {
    SERVER,
    CLIENT
}

// create or open a sem
pub fn open(sem_name: &str, access_type: AccessType) -> io::Result<*mut sem_t> {
    let sem_name = CString::new(sem_name).unwrap();
    let sem: *mut sem_t = match access_type {
        AccessType::SERVER =>  unsafe {
            sem_open(
                sem_name.as_ptr(),
                O_CREAT | O_EXCL,
                PERMISSIONS,
                1 // initial value
            )
        },
        AccessType::CLIENT => unsafe {
            sem_open(
                sem_name.as_ptr(),
                0,
            )
        },
    };

    if sem == SEM_FAILED {
        let errno: i32 = unsafe { *__error() };
        return match errno {
            EACCES => Err(Error::new(ErrorKind::PermissionDenied, "sem_open >> permission denied")),
            EINVAL => Err(Error::new(ErrorKind::InvalidInput, "sem_open >> invalid argument")),
            ENOENT => Err(Error::new(ErrorKind::InvalidInput, "sem_open >> no such file or directory")),
            ENOMEM => Err(Error::new(ErrorKind::OutOfMemory, "sem_open >> insufficient memory")),
            EEXIST =>Err(Error::new(ErrorKind::AlreadyExists, "sem_open >> a semaphore with this name already exists")),
            ENAMETOOLONG => Err(Error::new(ErrorKind::InvalidInput, "sem_open >> name too long")),
            _ => Err(Error::new(ErrorKind::Other, format!("sem_close >> an unknown error occurred: errno = {}", errno))),
        }
    }

    Ok(sem)
}

// wait on a sem object
pub fn wait(sem: *mut sem_t) -> io::Result<()> {
    // TODO: handle sem_wait failure
    info!(">> trying to aquire lock");
    let aquired = unsafe { sem_wait(sem) == 0 };
    info!(">> aquired lock: {aquired}");

    Ok(())
}

// post on a sem object
pub fn post(sem: *mut sem_t) -> io::Result<()> {
    // TODO: handle sem_post failure
    info!(">> releasing lock");
    let released = unsafe {sem_post(sem) == 0};
    info!(">> lock released: {released}");

    Ok(())
}

// close a sem object
pub fn close(sem: *mut sem_t) -> io::Result<()> {
  unsafe {
      if sem_close(sem) == -1 {
          let errno: i32 = *__error();
          return match errno {
              EINVAL => Err(Error::new(ErrorKind::InvalidInput, "sem_close >> sem is not a valid semaphore")),
              _ => Err(Error::new(ErrorKind::Other, format!("sem_close >> an unknown error occurred: errno = {}", errno))),
          }
      }
  }

  Ok(())
}

// remove a sem object
pub fn destroy(name: &str)  -> io::Result<()> {
    let sem_name = CString::new(name).unwrap();

    unsafe {
        if sem_unlink(sem_name.as_ptr()) == -1 {
            let errno: i32 = *__error();
            return match errno {
                EACCES => Err(Error::new(ErrorKind::PermissionDenied, "sem_unlink >> permission denied")),
                ENAMETOOLONG => Err(Error::new(ErrorKind::InvalidInput, "sem_unlink >> name too long")),
                ENOENT => Err(Error::new(ErrorKind::InvalidInput, "sem_unlink >> no semaphore with the given name")),
                _ => Err(Error::new(ErrorKind::Other, format!("sem_unlink >> an unknown error occurred: errno = {}", errno))),
            }
        }
    }

    Ok(())
}
