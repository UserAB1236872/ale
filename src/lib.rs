#![cfg_attr(feature="use_clippy", feature(plugin))]
#![cfg_attr(feature="use_clippy", plugin(clippy))]
extern crate rustc_serialize;
extern crate libc;

pub mod ffi;
mod game;
pub mod serialize;
pub use self::game::{Game,AleState,AleSystemState};

use ::ffi::*;
use ::libc::c_int;
use ::game::protected::Protected;
use std::ffi::{CStr,CString};
use std::ops::Drop;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, RustcEncodable, RustcDecodable)]
pub struct Action(pub i32);

pub struct Ale {
    p: *mut AleInterface
}

// ALE is not thread safe at the moment, so we need to ensure only one exists
static mut INSTANCE_EXISTS: AtomicBool = ATOMIC_BOOL_INIT;
const ALE_ERROR: &'static str = r#"An ALE instance already exists. 
The ALE currently uses global statics and is not thread safe, if you need multiple instances use a script to start multiple, separate processes.
If you need to run multiple ALEs in sequence on separate threads, arrange the synchronization yourself (e.g. mutexes or sending over a channel).
"#;

unsafe impl Send for Ale {}
unsafe impl Sync for Ale {}

impl Ale {
    pub fn new() -> Ale {
        use std::sync::atomic::Ordering;
        unsafe {
            assert!(!INSTANCE_EXISTS.swap(true, Ordering::SeqCst), ALE_ERROR);
        }
        Ale {
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

    pub unsafe fn from_raw_ptr(p: *mut AleInterface) -> Self {
        Ale {
            p: p,
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        unsafe {
            let key = CString::new(key).unwrap();
            getBool(self.p, key.as_ptr()) != 0
        }
    }

    pub fn get_int(&self, key: &str) -> i32 {
        unsafe {
            let key = CString::new(key).unwrap();
            getInt(self.p, key.as_ptr())
        }
    }

    pub fn get_float(&self, key: &str) -> f32 {
        unsafe {
            let key = CString::new(key).unwrap();
            getFloat(self.p, key.as_ptr())
        }
    }

    pub fn set_string(&mut self, key: &str, val: &str) {
        unsafe {
            let key = CString::new(key).unwrap();
            let val = CString::new(val).unwrap();

            setString(self.p, key.as_ptr(), val.as_ptr());
        }
    }

    pub fn set_bool(&mut self, key: &str, val: bool) {
        unsafe {
            let key = CString::new(key).unwrap();

            setBool(self.p, key.as_ptr(), val as c_int);
        }
    }

    pub fn set_int(&mut self, key: &str, val: i32) {
        unsafe {
            let key = CString::new(key).unwrap();

            setInt(self.p, key.as_ptr(), val);
        }
    }

    pub fn set_float(&mut self, key: &str, val: f32) {
        unsafe {
            let key = CString::new(key).unwrap();

            setFloat(self.p, key.as_ptr(), val);
        }
    }

    /// load_rom loads a rom from the given file name.
    /// This consumes the ALE interface and yields a game (because only one
    /// may be active at a time). The base ALE can be retrieved from the game.
    pub fn load_rom(self, file_name: &str) -> Game {
        unsafe {
            let file_name = CString::new(file_name).unwrap();

            loadROM(self.p, file_name.as_ptr());
        }

        Game::new(self, file_name.to_owned())
    }

}

impl Drop for Ale {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        unsafe { 
            ALE_del(self.p);
            INSTANCE_EXISTS.store(false, Ordering::Relaxed);
        }
    }
}