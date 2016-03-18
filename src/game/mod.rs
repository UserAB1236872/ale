use ::libc::c_int;
use std::convert::Into;
use std::ops::{Deref,DerefMut};
use std::ffi::CString;
use ::Action;
use ::Ale;

use ::rustc_serialize::{Encoder,Encodable,Decoder,Decodable};
use ::ffi::*;

pub mod serialize;

mod state;
pub use self::state::{AleState,AleSystemState};
use self::state::protected::Protected;

pub struct Game {
    ale: Ale,
    rom_path: String,
}

unsafe impl Send for Game {}
unsafe impl Sync for Game {}

impl Game {

    /// Changes the game by loading a new ROM. This consumes the current game
    /// and returns a new one with a reference to the same underlying ALE environment.
    pub fn change_game(self, file_name: &str) -> Game {
        self.ale.load_rom(file_name)
    }

    pub fn change_game_in_place(&mut self, file_name: &str) {
        unsafe {
            let file_name = CString::new(file_name).unwrap();
            loadROM(self.ale.p, file_name.as_ptr());
        }
    }

    pub fn act(&mut self, action: Action) -> i32 {
        unsafe {
            let Action(action) = action;

            act(self.ale.p, action)
        }
    }

    pub unsafe fn from_raw_ptr(p: *mut AleInterface) -> Self {
        Game {
            ale: Ale::from_raw_ptr(p),
            rom_path: "".to_owned(),
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
    pub fn reset(&mut self) {
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
    pub fn screen_dimensions(&self) -> (i32, i32) {
        unsafe {
            (getScreenWidth(self.ale.p), getScreenHeight(self.ale.p))
        }
    }

    pub fn screen_in_buf(&self, buf: &mut Vec<u8>) {
        unsafe {
            let (width, height) = self.screen_dimensions();
            let cap = buf.capacity();
            if cap < (width * height) as usize {
                buf.reserve_exact((width * height) as usize - cap);
            }

            buf.set_len((width * height) as usize);

            getScreen(self.ale.p, buf.as_mut_ptr());
        }
    }

    pub fn screen(&self) -> Vec<u8> {
        let (width, height) = self.screen_dimensions();
        let mut buf = Vec::<u8>::with_capacity((width * height) as usize);

        self.screen_in_buf(&mut buf);

        buf
    }

    pub fn screen_rgb_in_buf(&self, buf: &mut Vec<u8>) {
        unsafe {
            let (width, height) = self.screen_dimensions();
            let cap = buf.capacity();
            if cap < (width * height) as usize {
                buf.reserve_exact((width * height * 3) as usize - cap);
            }

            buf.set_len((width * height * 3) as usize);

            getScreenRGB(self.ale.p, buf.as_mut_ptr());
        }
    }

    pub fn screen_rgb(&self) -> Vec<u8> {
        let (width, height) = self.screen_dimensions();
        let mut buf = Vec::<u8>::with_capacity((width * height * 3) as usize);

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

    pub fn save_state(&mut self) {
        unsafe {
            saveState(self.ale.p);
        }
    }

    pub fn load_state(&mut self) {
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

    pub fn clone_state(&self) -> AleState {
        unsafe { AleState::new(cloneState(self.ale.p)) }
    }

    pub fn clone_system_state(&self) -> AleSystemState {
        unsafe { AleSystemState::new(cloneSystemState(self.ale.p)) }
    }

    pub fn restore_from_cloned_state(&mut self, s: &AleState) {
        unsafe {
            restoreState(self.ale.p, s.s());
        }
    }

    pub fn restore_from_cloned_system_state(&mut self, s: &AleSystemState) {
        unsafe {
            restoreSystemState(self.ale.p, s.s());
        }
    }

    pub fn rom(&self) -> serialize::Rom {
        serialize::Rom::new(&self.rom_path)
    }
}

impl Encodable for Game {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        use self::serialize::Rom;
        
        try!(self.rom_path.encode(s));
        let rom = Rom::new(&self.rom_path);
        try!(rom.encode(s));

        self.clone_system_state().encode(s)
    }
}

impl Decodable for Game {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        use self::serialize::GameDecoder;
        let prelim = try!(GameDecoder::decode(d));
        Ok(prelim.decode_game())
    }
}

impl Into<Ale> for Game {
    fn into(self) -> Ale {
        self.ale
    }
}

impl Deref for Game {
    type Target=Ale;

    fn deref(&self) -> &Ale {
        &self.ale
    }
}

impl DerefMut for Game {
    fn deref_mut(&mut self) -> &mut Ale {
        &mut self.ale
    }
}


pub mod protected {
    use ::Ale;
    use super::Game;

    pub trait Protected {
        fn new(ale: Ale, path: String) -> Self;
    } 

    impl Protected for Game {
        fn new(ale: Ale, path: String) -> Self {
            Game { ale: ale, rom_path: path }
        }
    }
}