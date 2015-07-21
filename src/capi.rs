extern crate libc;
use std::ffi::{CStr, CString};
use std::ops::Drop;

use self::libc::{c_char, c_int, c_float};

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

    pub fn get_string(&self, key: &str) -> &str {
        use std::str::from_utf8;

        unsafe {
            let key = CString::new(key).unwrap();
            let cstr = CStr::from_ptr(getString(self.p, key.as_ptr()));

            from_utf8(cstr.to_bytes()).unwrap()
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        unsafe {
            let key = CString::new(key).unwrap();
            getBool(self.p, key.as_ptr()) == 0
        }
    }

    pub fn get_int(&self, key: &str) -> isize {
        unsafe {
            let key = CString::new(key).unwrap();
            getInt(self.p, key.as_ptr()) as isize
        }
    }

    pub fn get_float(&self, key: &str) -> f64 {
        unsafe {
            let key = CString::new(key).unwrap();
            getFloat(self.p, key.as_ptr()) as f64
        }
    }

    pub fn set_string(&self, key: &str, val: &str) {
        unsafe {
            let key = CString::new(key).unwrap();
            let val = CString::new(val).unwrap();

            setString(self.p, key.as_ptr(), val.as_ptr());
        }
    }

    pub fn set_bool(&self, key: &str, val: bool) {
        unsafe {
            let key = CString::new(key).unwrap();

            setBool(self.p, key.as_ptr(), val as c_int);
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


    // Getters
    fn getString(i: *mut ale_interface, key: *const c_char) -> *const c_char;
    fn getBool(i: *mut ale_interface, key: *const c_char) -> c_int;
    fn getInt(i: *mut ale_interface, key: *const c_char) -> c_int;
    fn getFloat(i: *mut ale_interface, key: *const c_char) -> c_float;

    // Setters
    fn setString(i: *mut ale_interface, key: *const c_char, val: *const c_char);
    fn setBool(i: *mut ale_interface, key: *const c_char, val: c_int);
}