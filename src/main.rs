// main.rs
#![allow(dead_code)]
#![allow(unused)]
// get rid of this during compile time, it's just annoying during prototyping

use rand::{Rng, random_bool};
//#region
pub const X_MAX: usize      = 10;
pub const Y_MAX: usize      = 8;
pub const GRID_SIZE: usize  = X_MAX * Y_MAX;
pub const HOLE_PROBABILITY: f64 = 0.0; // 0 >= n >= 1
pub const PLAYER_NUMBER: usize = 2;
pub const NUMBANK_SIZE:  usize = usize::div_ceil(GRID_SIZE, PLAYER_NUMBER); // ceiling division!
//#endregion
/* maybe move the above constants into the struct for the grid */

fn main() {

}

pub const fn get_x(location: usize) -> usize {
    location % X_MAX
}
pub const fn get_y(location: usize) -> usize {
    location / X_MAX // this is floor division, apparently
}

struct Grid{
    values: [u8; GRID_SIZE],
    owners: [u8; GRID_SIZE],
    takens: [bool; GRID_SIZE],
    
    tile_num: u16,
    turn: u16,
}

impl Grid {
    fn init(&mut self){
        self.values = [0u8; GRID_SIZE];
        self.owners = [0u8; GRID_SIZE];
        self.turn = 0;

        if HOLE_PROBABILITY > 0.0 {
            let mut rng = rand::rng();
            self.takens = [false; GRID_SIZE]; // consider switching vec to arc[<t>]; look into it at least
            self.takens.fill_with(|| {
                random_bool(HOLE_PROBABILITY)
            });
        }

        else {
            self.takens  = [false; GRID_SIZE];
        }
        
    }

    fn add(&mut self, value: u8, owner: u8, location: usize){
        if self.takens[location] {
            panic!()
        }
        self.values[location] = value;
        self.owners[location] = owner;
        self.takens[location] = true;
        self.update_neighbors(value, owner, location);
        self.turn += 1;
    }
    
    fn update_neighbors(&mut self, value: u8, owner: u8, location: usize){
        let neighbors = self.get_neighbors(location);
        for neighbor in neighbors{
            let neighbor_owner = self.owners[neighbor];
            let neighbor_score = self.values[neighbor];
            if neighbor_owner == 0 {continue}
            
            else if neighbor_owner == owner {
                self.values[location] += 1;
            }

            else if neighbor_score < value {
                self.owners[location] = owner;
            }
            
        }

    }

    fn get_neighbors(&mut self, location: usize) -> Vec<usize>{
        // really verbose and probably not idiotomic 
        let is_odd = (get_y(location) % 2 == 1);
        let mut neighbors: Vec<usize> = vec![];

        if (location < GRID_SIZE) && !self.takens[location + 1] {
            neighbors.push(location + 1)}       // right neighbor
        
        if location > 0 && !self.takens[location - 1]{
            neighbors.push(location - 1)}       // left neighbor

        if location + X_MAX <= GRID_SIZE && !self.takens[location + X_MAX] {
            neighbors.push(location + X_MAX);   // bottom neighbor (1)
        }
        
        if location >= X_MAX && !self.takens[location - X_MAX]  {
            neighbors.push(location - X_MAX);   // top neighbor (1)
        }


        // maybe just add 1 if odd and subtract 1 if not, making the + or - 1 depend on is_odd
        if is_odd {
            if location >= X_MAX - 1 && !self.takens[location - X_MAX + 1] {
                neighbors.push(location - X_MAX + 1)
            }
            if location >= X_MAX + 1 && !self.takens[location + X_MAX + 1] {
                neighbors.push(location + X_MAX + 1)
            }
        }
        
        else {
            if location >= X_MAX - 1 && !self.takens[location - X_MAX - 1] {
                neighbors.push(location - X_MAX - 1)
            }
            if location >= X_MAX + 1&& !self.takens[location + X_MAX - 1] {
                neighbors.push(location + X_MAX - 1)
            }
        }
    
    return neighbors

    }

}

