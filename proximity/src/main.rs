// main.rs
#![allow(unused)]
// mod convert;
/* simulations / sec measured at start of game with 80 moves to start
Jan 12: 7600      simulations / second
Jan 14: 9500      simulations / second
Jan 15: 20,000    simulations / second  (wow! goal met already!)
Jan 17: 340,000   simulations / second  (achieved with multithreading)
Jan 22: 1,051,654 simulations / second  HOLY SHIT !!
Jan 23: 1,200,000 simulations / second

i think i'm reaching the limit, i'll try the error function thing

RUSTFLAGS="-C target-cpu=native" cargo build --release
*/

mod constants;
mod player;
mod grid;
mod game;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    
    game.game_loop();
    println!("{:?}", game.get_scores())
}
