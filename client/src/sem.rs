use std::io;

use libc::sem_t;

use utils::sem;

/// open a sem object
pub fn open(sem_name: &str) -> io::Result<*mut sem_t> {    
    sem::open(sem_name, sem::AccessType::CLIENT, -1)
}

/// wait on a sem object
pub fn wait(sem: *mut sem_t) -> io::Result<()> {
    sem::wait(sem)
}

/// post on a sem object
pub fn post(sem: *mut sem_t) -> io::Result<()> {
    sem::post(sem)
}

/// close a sem object
pub fn close(sem: *mut sem_t) -> io::Result<()> {
   sem::close(sem)
}