use std::os::raw::c_void;
use std::ptr::write;

use memoffset::offset_of;

#[repr(C, packed)]
#[derive(Debug)]
pub struct SharedMemory {
    // request buffer
    req_buffer: [i32; 10],
    req_front: usize,
    req_rear: usize,

    // response buffer
    res_buffer: [i32; 10],
    res_front: usize,
    res_rear: usize,
}

/// read from shm
pub fn shm_read(ptr: *mut c_void) -> usize {
    let shm_ptr = ptr as *mut SharedMemory;
    
    let offset: usize = offset_of!(SharedMemory, req_front);
    
    unsafe {
        let value_ptr: *mut usize=  (shm_ptr as *mut usize).add(offset);
        let msg: &usize = &*value_ptr;
        
        return *msg;
    };
}

/// write from shm
pub fn shm_write(ptr: *mut c_void, value: usize ) {

    let shm_ptr = ptr as *mut SharedMemory;
    let offset: usize = offset_of!(SharedMemory, req_front);
    
    unsafe {
        let value_ptr = (shm_ptr as *mut usize).add(offset);
    
        // overwites a memory location
        write(value_ptr,  value );
    }
}
