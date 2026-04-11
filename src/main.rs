// main.rs
#![allow(unused)]
// mod convert;
/* simulations / sec measured at start of game with 80 moves to start
Jan 12: 7600        simulations / second
Jan 14: 9500        simulations / second
Jan 15: 20,000      simulations / second  (wow! goal met already!)
Jan 17: 340,000     simulations / second (achieved with multithreading)
Jan 22: 1,051,654   simulations / second HOLY **** !!
Jan 23: ~1,200,000  simulations / second (11ns/move)
Jan 24: 1,600,000   simulations / second (added (array)vec of valid moves for faster selection)
Jan 29: 1,900,000   simulations / second (switched out own binary search with builtin) (6.5ns/move)
Jan 31: 2,200,000   simulations / second (branchless prediction in update_neighbors)
i think i'm reaching the limit, i'll try the error function thing to
make myself do fewer simulations for the same result

linux/macos RUSTFLAGS="-C target-cpu=native" cargo build --release
powershell $env:RUSTFLAGS="-C target-cpu=native"; cargo build --release
*/

mod constants;
mod player;
mod grid;
mod game;

use crate::game::Game;
// use clap::Args;

fn main() {
    crate::constants::init_config_from_args();
    let mut game = Game::new();
    
    game.game_loop();
    println!("{:?}", game.get_scores())
}
