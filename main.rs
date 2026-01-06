// main.rs

#![allow(dead_code)]
#![allow(unused)]

// get rid of this during compile time, it's just annoying during prototyping

use rand::{Rng, random_bool};

const X_MAX: usize     = 10;
const Y_MAX: usize     = 8;
const GRID_SIZE: usize = X_MAX * Y_MAX;

const HOLE_PROBABILITY: f64 = 0.1; // 0 >= n >= 1

const PLAYER_NUMBER: usize = 2;
const NUMBANK_SIZE: usize =
    usize::div_ceil(GRID_SIZE, PLAYER_NUMBER); // ceiling division!

/* maybe move the above constants into the struct for the grid */

struct Grid {
    values: [u8; GRID_SIZE],
    owners: [u8; GRID_SIZE],
    takens: [bool, GRID_SIZE]
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

impl Grid {
    fn init(&mut self) {
        self.values = [0u8; GRID_SIZE];
        self.owners = [0u8; GRID_SIZE];

        if HOLE_PROBABILITY > 0.0 {
            self.rng = rand::rng();

            self.takens = [false; GRID_SIZE]; // consider switching vec to arc[<t>]; look into it at least
            self.takens.fill_with(|| {
                random_bool(HOLE_PROBABILITY)
            });
        } else {
            self.takens = [false; GRID_SIZE];
        }
    }

    fn add(&mut self, value: u8, owner: u8, location: usize) {
        if self.takens[location] = True {
            panic!()
        }

        self.values[location] = location
        self.owners[location] = owner
        self.takens[location] = true
    }
}
