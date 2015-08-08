extern crate libc;

use std::ffi::{CStr, CString};
use std::ops::Drop;

use self::libc::{c_char, c_int, c_float, c_uchar};

use std::convert::Into;

#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]
pub struct Action(pub i32);

pub struct ALE {
    p: *mut AleInterface
}

enum AleInterface{}

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

    pub fn set_int(&self, key: &str, val: i32) {
        unsafe {
            let key = CString::new(key).unwrap();

            setInt(self.p, key.as_ptr(), val);
        }
    }

    pub fn set_float(&self, key: &str, val: f32) {
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

    pub fn act(&self, action: Action) -> i32 {
        unsafe {
            let Action(action) = action;

            act(self.ale.p, action)
        }
    }

    /// This reports whether or not the game is over. This is equivalent to the C API wrapper's
    /// game_over function.
    pub fn is_over(&self) -> bool {
        unsafe {
            game_over(self.ale.p) != 0
        }
    }

    /// Resets the current game. This is equivalent to the C API wrapper's
    /// reset_game function.
    pub fn reset(&self) {
        unsafe {
            reset_game(self.ale.p);
        }
    }

    pub fn legal_action_set(&self) -> Vec<Action> {
        unsafe {
            let size = getLegalActionSize(self.ale.p) as usize;
            let mut buf = Vec::<c_int>::with_capacity(size);

            getLegalActionSet(self.ale.p, buf.as_mut_ptr());

            buf.set_len(size);

            let mut actions = Vec::<Action>::with_capacity(size);

            for action in buf.into_iter() {
                actions.push(Action(action));
            }

            actions
        }
    }

    pub fn minimal_action_set(&self) -> Vec<Action> {
        unsafe {
            let size = getMinimalActionSize(self.ale.p) as usize;
            let mut buf = Vec::<c_int>::with_capacity(size);

            getMinimalActionSet(self.ale.p, buf.as_mut_ptr());

            buf.set_len(size);

            let mut actions = Vec::<Action>::with_capacity(size);

            for action in buf.into_iter() {
                actions.push(Action(action));
            }

            actions
        }
    }

    pub fn frame_number(&self) -> i32 {
        unsafe {
            getFrameNumber(self.ale.p)
        }
    }

    pub fn lives(&self) -> i32 {
        unsafe {
            lives(self.ale.p)
        }
    }

    pub fn episode_frame_number(&self) -> i32 {
        unsafe {
            getEpisodeFrameNumber(self.ale.p)
        }
    }

    /// Gets the screen dimensions and returns them as a tuple of
    /// (width,height)
    pub fn screen_dimensions(&self) -> (i32,i32) {
        unsafe {
            (getScreenWidth(self.ale.p), getScreenHeight(self.ale.p))
        }
    }

    pub fn screen_in_buf(&self, buf: &mut Vec<u8>) {
        unsafe {
            let (width,height) = self.screen_dimensions();
            let cap = buf.capacity();
            if cap < (width*height) as usize {
                buf.reserve_exact((width*height) as usize - cap);
            }

            buf.set_len((width*height) as usize);

            getScreen(self.ale.p, buf.as_mut_ptr());
        }
    }

    pub fn screen(&self) -> Vec<u8> {
        let (width,height) = self.screen_dimensions();
        let mut buf = Vec::<u8>::with_capacity( (width*height) as usize);

        self.screen_in_buf(&mut buf);

        buf
    }

    pub fn screen_rgb_in_buf(&self, buf: &mut Vec<u8>) {
        unsafe {
            let (width,height) = self.screen_dimensions();
            let cap = buf.capacity();
            if cap < (width*height) as usize {
                buf.reserve_exact((width*height) as usize - cap);
            }

            buf.set_len((width*height) as usize);

            getScreenRGB(self.ale.p, buf.as_mut_ptr());
        }
    }

    pub fn screen_rgb(&self) -> Vec<u8> {
        let (width,height) = self.screen_dimensions();
        let mut buf = Vec::<u8>::with_capacity( (width*height) as usize);

        self.screen_rgb_in_buf(&mut buf);

        buf
    }

    pub fn ram_size(&self) -> i32 {
        unsafe {
            getRAMSize(self.ale.p)
        }
    }

    pub fn ram_in_buf(&self, buf: &mut Vec<u8>) {
        unsafe {
            let size = self.ram_size() as usize;
            let cap = buf.capacity();
            if cap < size {
                buf.reserve_exact(size - cap);
            }

            buf.set_len(size);

            getRAM(self.ale.p, buf.as_mut_ptr());
        }
    }

    pub fn ram(&self) -> Vec<u8> {
        let size = self.ram_size() as usize;
        let mut buf = Vec::<u8>::with_capacity(size);

        self.ram_in_buf(&mut buf);

        buf
    }

    pub fn save_state(&self) {
        unsafe {
            saveState(self.ale.p);
        }
    }

    pub fn load_state(&self) {
        unsafe {
            loadState(self.ale.p);
        }
    }

    pub fn save_screen_png(&self, file_name: &str) {
        unsafe {
            let file_name = CString::new(file_name).unwrap();

            saveScreenPNG(self.ale.p, file_name.as_ptr());
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

    fn ALE_new() -> *mut AleInterface;
    fn ALE_del(i: *mut AleInterface);


    // Getters
    fn getString(i: *mut AleInterface, key: *const c_char) -> *const c_char;
    fn getBool(i: *mut AleInterface, key: *const c_char) -> c_int;
    fn getInt(i: *mut AleInterface, key: *const c_char) -> c_int;
    fn getFloat(i: *mut AleInterface, key: *const c_char) -> c_float;

    // Setters
    fn setString(i: *mut AleInterface, key: *const c_char, val: *const c_char);
    fn setBool(i: *mut AleInterface, key: *const c_char, val: c_int);
    fn setInt(i: *mut AleInterface, key: *const c_char, val: c_int);
    fn setFloat(i: *mut AleInterface, key: *const c_char, val: c_float);

    fn loadROM(i: *mut AleInterface, file_name: *const c_char);

    // General emulation
    fn act(i: *mut AleInterface, action: c_int) -> c_int;
    fn game_over(i: *mut AleInterface) -> c_int;
    fn reset_game(i: *mut AleInterface);
    
    // Action getters
    fn getLegalActionSet(i: *mut AleInterface, actions: *mut c_int);
    fn getLegalActionSize(i: *mut AleInterface) -> c_int;
    fn getMinimalActionSet(i: *mut AleInterface, actions: *mut c_int);
    fn getMinimalActionSize(i: *mut AleInterface) -> c_int;

    fn getFrameNumber(i: *mut AleInterface) -> c_int;
    fn lives(i: *mut AleInterface) -> c_int;
    fn getEpisodeFrameNumber(i: *mut AleInterface) -> c_int;

    // Screen functions
    fn getScreenWidth(i: *mut AleInterface) -> c_int;
    fn getScreenHeight(i: *mut AleInterface) -> c_int;
    fn getScreen(i: *mut AleInterface, buf: *const c_uchar);
    fn getScreenRGB(i: *mut AleInterface, buf: *const c_uchar);

    // RAM
    fn getRAMSize(i: *mut AleInterface) -> c_int;
    fn getRAM(i: *mut AleInterface, buf: *const c_uchar);

    // State and screen saving
    fn saveState(i: *mut AleInterface);
    fn loadState(i: *mut AleInterface);
    fn saveScreenPNG(i: *mut AleInterface, file_name: *const c_char);
}