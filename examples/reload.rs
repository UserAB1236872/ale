extern crate ale;

use ale::{ALE};

fn main() {
	let (state,screen) = {
		let game = ALE::new().load_rom("space_invaders.bin");

		(game.clone_system_state(),game.screen_rgb())
	};

	let mut game = ALE::new().load_rom("space_invaders.bin");
	game.restore_from_cloned_system_state(&state);
	assert_eq!(screen, game.screen_rgb());
}