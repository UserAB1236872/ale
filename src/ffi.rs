extern crate libc;

use ::libc::{c_char, c_int, c_float, c_uchar};

pub enum AleInterface {}
pub enum CAleState {}

#[link(name = "ale_c")]
extern {
    // Creation/Deletion functions

    pub fn ALE_new() -> *mut AleInterface;
    pub fn ALE_del(i: *mut AleInterface);

    // Getters
    pub fn getString(i: *mut AleInterface, key: *const c_char) -> *const c_char;
    pub fn getBool(i: *mut AleInterface, key: *const c_char) -> c_int;
    pub fn getInt(i: *mut AleInterface, key: *const c_char) -> c_int;
    pub fn getFloat(i: *mut AleInterface, key: *const c_char) -> c_float;

    // Setters
    pub fn setString(i: *mut AleInterface, key: *const c_char, val: *const c_char);
    pub fn setBool(i: *mut AleInterface, key: *const c_char, val: c_int);
    pub fn setInt(i: *mut AleInterface, key: *const c_char, val: c_int);
    pub fn setFloat(i: *mut AleInterface, key: *const c_char, val: c_float);

    pub fn loadROM(i: *mut AleInterface, file_name: *const c_char);

    // General emulation
    pub fn act(i: *mut AleInterface, action: c_int) -> c_int;
    pub fn game_over(i: *mut AleInterface) -> c_int;
    pub fn reset_game(i: *mut AleInterface);

    // Action getters
    pub fn getLegalActionSet(i: *mut AleInterface, actions: *mut c_int);
    pub fn getLegalActionSize(i: *mut AleInterface) -> c_int;
    pub fn getMinimalActionSet(i: *mut AleInterface, actions: *mut c_int);
    pub fn getMinimalActionSize(i: *mut AleInterface) -> c_int;

    pub fn getFrameNumber(i: *mut AleInterface) -> c_int;
    pub fn lives(i: *mut AleInterface) -> c_int;
    pub fn getEpisodeFrameNumber(i: *mut AleInterface) -> c_int;

    // Screen functions
    pub fn getScreenWidth(i: *mut AleInterface) -> c_int;
    pub fn getScreenHeight(i: *mut AleInterface) -> c_int;
    pub fn getScreen(i: *mut AleInterface, buf: *const c_uchar);
    pub fn getScreenRGB(i: *mut AleInterface, buf: *const c_uchar);

    // RAM
    pub fn getRAMSize(i: *mut AleInterface) -> c_int;
    pub fn getRAM(i: *mut AleInterface, buf: *const c_uchar);

    // State and screen saving
    pub fn saveState(i: *mut AleInterface);
    pub fn loadState(i: *mut AleInterface);
    pub fn saveScreenPNG(i: *mut AleInterface, file_name: *const c_char);

    // Serialization
    pub fn cloneState(i: *mut AleInterface) -> *mut CAleState;
    pub fn restoreState(i: *mut AleInterface, s: *mut CAleState);
    pub fn cloneSystemState(i: *mut AleInterface) -> *mut CAleState;
    pub fn restoreSystemState(i: *mut AleInterface, s: *mut CAleState);

    pub fn deleteState(s: *mut CAleState);

    pub fn encodeState(s: *mut CAleState, buf: *mut c_char) -> *const c_char;
    pub fn encodeStateLen(s: *mut CAleState) -> i32;
    pub fn decodeState(state: *const c_char, len: c_int) -> *mut CAleState;
}