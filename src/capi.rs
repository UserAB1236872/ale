extern crate libc;
use std::ffi::CString;
use std::ops::Drop;

pub struct ALE {
    p: *mut ale_interface
}

#[repr(C)]
struct ale_interface;

impl ALE {
    pub fn new() -> ALE {
        ALE {
            p: unsafe { ALE_new() }
        }
    }
}

impl Drop for ALE {
    fn drop(&mut self) {
        unsafe { ALE_del(self.p) }
    }
}

#[link(name="ale_c")]
extern {
    // Creation/Deletion functions

    fn ALE_new() -> *mut ale_interface;
    fn ALE_del(i: *mut ale_interface);


    // Utility functions
}