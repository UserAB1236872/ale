use std::path::{Path,PathBuf};
use std::fs::{File};
use std::io::Write;
use rustc_serialize::{Decoder,Decodable};
use ::{Ale,AleSystemState};
use super::Game;

/// A GameDecoder allows you to set ALE properties before
/// restoring a decoded game state. Note that altering some properties
/// such as repeat_action_probability may affect replays. It's recommended
/// to only use this to alter things like display_screen.
pub struct GameDecoder {
	pub rom_path: PathBuf,
	ale: Ale,
	initial_state: AleSystemState,
	romfile: Vec<u8>,
}

impl GameDecoder {
	pub fn get_int(&self, key: &str) -> i32 {
		self.ale.get_int(key)
	}

	pub fn get_bool(&self, key: &str) -> bool {
		self.ale.get_bool(key)
	}

	pub fn get_string(&self, key: &str) -> &str {
		self.ale.get_string(key)
	}

	pub fn get_float(&self, key: &str) -> f32 {
		self.ale.get_float(key)
	}

	pub fn set_int(&mut self, key: &str, val: i32) {
		self.ale.set_int(key, val);
	}

	pub fn set_bool(&mut self, key: &str, val: bool) {
		self.ale.set_bool(key, val);
	}

	pub fn set_string(&mut self, key: &str, val: String) {
		self.ale.set_string(key, &*val);
	}

	pub fn set_float(&mut self, key: &str, val: f32) {
		self.ale.set_float(key, val);
	}

	pub fn decode_game(self) -> Game {
		let path = self.rom_path.as_path();
		if !path.exists() {
			match File::create(path) {
					Ok(mut file) => { file.write_all(self.romfile.as_slice()).expect("Could not find or write ROM file") },
					Err(err) => panic!(err),
			};
		}

		let mut game = self.ale.load_rom(path.to_str().unwrap());
		game.restore_from_cloned_system_state(&self.initial_state);

		game
	}
}

impl Decodable for GameDecoder {
	fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
		Ok(GameDecoder{
			rom_path: Path::new(&try!(String::decode(d))).to_path_buf(),
			romfile: try!(Vec::<u8>::decode(d)),
			ale: Ale::new(),
			initial_state: try!(AleSystemState::decode(d)),
		})
	}
}