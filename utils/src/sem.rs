use libc::{sem_t, sem_wait, sem_post};

pub struct Semaphore {
    sem: *mut sem_t,
}

impl Semaphore {
    pub fn new(sem: *mut sem_t) -> Self {
        Semaphore { sem }
    }

    pub fn wait(&self) {
        unsafe { sem_wait(self.sem) };
    }

    pub fn post(&self) {
        unsafe { sem_post(self.sem) };
    }
}