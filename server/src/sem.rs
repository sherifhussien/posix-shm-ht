use std::io;

use libc::sem_t;

use utils::sem;

/// create and open a sem
pub fn open(sem_name: &str, initial_value: isize) -> io::Result<*mut sem_t> {
    sem::open(sem_name, sem::AccessType::SERVER, initial_value)
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

/// remove a sem object
pub fn destroy(name: &str) -> io::Result<()> {
    sem::destroy(name)
}