extern crate libc;
use std::ffi::{CStr, CString};
use std::ops::Drop;

use self::libc::{c_char, c_int, c_float};

use std::convert::Into;

pub struct Action(isize);

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
            getBool(self.p, key.as_ptr()) != 0
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

    pub fn set_int(&self, key: &str, val: isize) {
        unsafe {
            let key = CString::new(key).unwrap();

            setInt(self.p, key.as_ptr(), val as c_int);
        }
    }

    pub fn set_float(&self, key: &str, val: f64) {
        unsafe {
            let key = CString::new(key).unwrap();

            setFloat(self.p, key.as_ptr(), val as c_float);
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

        Game::new(self)
    } 
}

impl Drop for ALE {
    fn drop(&mut self) {
        unsafe { ALE_del(self.p) }
    }
}

pub struct Game {
    ale: ALE
}

impl Game {
    fn new(ale: ALE) -> Game {
        Game{ale: ale}
    }

    /// Changes the game by loading a new ROM. This consumes the current game
    /// and returns a new one with a reference to the same underlying ALE environment.
    pub fn change_game(self, file_name: &str) -> Game {
        self.ale.load_rom(file_name)
    }

    pub fn act(&self, action: Action) {
        unsafe {
            let Action(action) = action;

            act(self.ale.p, action as c_int);
        }
    }

    /// This reports whether or not the game is over. This is equivalent to the C API wrapper's
    /// game_over function.
    pub fn is_over(&self) -> bool {
        unsafe {
            game_over(self.ale.p) != 0
        }
    }
}

impl Into<ALE> for Game {
    fn into(self) -> ALE {
        self.ale
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
    fn setInt(i: *mut ale_interface, key: *const c_char, val: c_int);
    fn setFloat(i: *mut ale_interface, key: *const c_char, val: c_float);

    fn loadROM(i: *mut ale_interface, file_name: *const c_char);

    // General emulation
    fn act(i: *mut ale_interface, action: c_int);
    fn game_over(i: *mut ale_interface) -> c_int;
}