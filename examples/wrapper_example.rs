#![feature(convert)]

extern crate ale;
extern crate rand;

use ale::{ALE};
use std::env::args;
use rand::{Rng};

const EPISODES: i32 = 10;

fn main() {
    let mut args = args();
    let rom_name = match args.nth(1) {
        None => { panic!("You need to supply an Atari rom name!"); },
        Some(fname) => fname,
    };

    let ale = ALE::new();
    ale.set_int("random_seed", 123);
    ale.set_float("repeat_action_probability", 0.25);

    let game = ale.load_rom(rom_name.as_str());

    let legal_actions = game.legal_action_set();
    let mut rng = rand::thread_rng();

    for episode in 0..EPISODES {
        let mut total_reward = 0;
        while !game.is_over() {
            let a = rng.choose(legal_actions.as_slice());
            let a = match a {
                Some(a) => a,
                None => { panic!("No actions available to select"); },
            };
            total_reward += game.act(*a);
        }

        println!("Episode {} ended with score {}.", episode, total_reward);
        game.reset();
    }
}