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

    let screenshot = match args.next() {
        None => false,
        Some(_) => true,
    };

    let ale = ALE::new();

    let game = ale.load_rom(rom_name.as_str());

    let legal_actions = game.legal_action_set();
    let mut rng = rand::thread_rng();

    let mut i = 0;
    for episode in 0..EPISODES {
        let mut total_reward = 0;
        if screenshot {
            game.save_screen_png(format!("./pictures/{}.png", i).as_str());
        }
        while !game.is_over() {
            let a = rng.choose(legal_actions.as_slice());
            let a = match a {
                Some(a) => a,
                None => { panic!("No actions available to select"); },
            };
            total_reward += game.act(*a);
            i += 1;
            if screenshot {
                game.save_screen_png(format!("./pictures/{}.png", i).as_str());
            }
        }

        println!("Episode {} ended with score {}.", episode, total_reward);
        game.reset();
        i += 1;
    }
}