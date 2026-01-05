// main.rs
#![allow(dead_code)]
#![allow(unused)]
// get rid of this during compile time, it's just annoying during prototyping

use rand::{Rng, random_bool};

const X_MAX: usize      = 10;
const Y_MAX: usize      = 8;
const GRID_SIZE: usize  = X_MAX * Y_MAX;

const HOLE_PROBABILITY: f64    = 0.1; // 0 >= n >= 1

const PLAYER_NUMBER: usize = 2;
const NUMBANK_SIZE:  usize = usize::div_ceil(GRID_SIZE, PLAYER_NUMBER); // ceiling division!
/* maybe move the above constants into the struct for the grid */

struct Grid{
    values: [u8; GRID_SIZE],
    owners: [u8; GRID_SIZE],
}

fn main() {
    initialize_state();
}

pub const fn get_x(location: usize) -> usize {
    location % X_MAX
}
pub const fn get_y(location: usize) -> usize {
    location / X_MAX // this is floor division, apparently
}



fn initialize_state(){
    let mut values = vec![0u8; GRID_SIZE]; 
    let mut owners = vec![0u8; GRID_SIZE];

    if HOLE_PROBABILITY > 0.0 {
        let mut rng = rand::rng();
        let mut taken = vec![false; GRID_SIZE]; // consider switching vec to arc[<t>]; look into it at least
        taken.fill_with(|| {
            random_bool(HOLE_PROBABILITY)
        });
        
        }
    
    else {
        let mut taken  = vec![false; GRID_SIZE];
    }
}

fn add_tile(location: usize, value_vector: Vec<u8>, owner_vector: Vec<u8>, valid_vector: Vec<bool>, value: u8, owner: u8){
    println!("placeholder {}", location)
}
