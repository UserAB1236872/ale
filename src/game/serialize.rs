use std::path::{Path,PathBuf};
use std::convert::AsRef;
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
	pub rom: Rom,
	ale: Ale,
	initial_state: AleSystemState,
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
		let backup_path = self.rom.create();

		let mut game = self.ale.load_rom(backup_path.to_str().expect("Could not decode path to string"));
		game.restore_from_cloned_system_state(&self.initial_state);

		game
	}
}

impl Decodable for GameDecoder {
	fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
		Ok(GameDecoder{
			rom: try!(Rom::decode(d)),
			ale: Ale::new(),
			initial_state: try!(AleSystemState::decode(d)),
		})
	}
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct Rom {
	pub rom_path: PathBuf,
	pub data: Vec<u8>,
}

impl Rom {
	pub fn new<P: AsRef<Path>>(p: P) -> Self {
		use std::io::Read;
		let path = p.as_ref().to_path_buf();

        let mut file = File::open(&path).expect("Could not open ROM file");
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf).expect("Could not read ROM file");

        Rom {
        	rom_path: path,
        	data: buf,
        }
	}

	pub fn create(&self) -> PathBuf {
		use std::fs;

		let filename = self.rom_path.file_name().expect("Rom didn't have an actual file name?")
						   .to_str().expect("Filename was not valid unicode");

		let backup_path = Path::new(&format!("./ROMs/{}", filename)).to_path_buf();

		if !backup_path.exists() {	
			fs::create_dir_all("./ROMs").expect("Could not create ROM directory");
			match File::create(&backup_path) {
					Ok(mut file) => { file.write_all(self.data.as_slice()).expect("Could not find or write ROM file") },
					Err(err) => panic!(err),
			};
		};

		backup_path
	}
}

/// The old game decoder
pub struct LegacyGameDecoder {
	pub rom_path: PathBuf,
	ale: Ale,
	initial_state: AleSystemState,
	romfile: Vec<u8>,
}

impl LegacyGameDecoder {
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
		use std::fs;

		let path = self.rom_path.as_path();
		let filename = path.file_name().expect("Rom didn't have an actual file name?")
						   .to_str().expect("Filename was not valid unicode");

		let backup_path = format!("./ROMs/{}", filename);

		if !path.exists() {	
			fs::create_dir_all("./ROMs").expect("Could not create ROM directory");
			match File::create(&backup_path) {
					Ok(mut file) => { file.write_all(self.romfile.as_slice()).expect("Could not find or write ROM file") },
					Err(err) => panic!(err),
			};
		}

		let mut game = self.ale.load_rom(&backup_path);
		game.restore_from_cloned_system_state(&self.initial_state);

		game
	}
}

impl Decodable for LegacyGameDecoder {
	fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
		Ok(LegacyGameDecoder{
			rom_path: Path::new(&try!(String::decode(d))).to_path_buf(),
			romfile: try!(Vec::<u8>::decode(d)),
			ale: Ale::new(),
			initial_state: try!(AleSystemState::decode(d)),
		})
	}
}