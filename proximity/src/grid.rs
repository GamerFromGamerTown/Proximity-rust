use arrayvec::ArrayVec;
use rand::{random_bool, rng, seq::SliceRandom};

use crate::constants::{
    get_valid_moves_from_takens, location_to_x, location_to_y, EVEN_OFFSETS, GRID_ISIZE, GRID_SIZE,
    HOLE_PROBABILITY, ODD_OFFSETS, X_MAX,
};

#[derive(Clone)]
pub(crate) struct Grid {
    pub(crate) values: [u8; GRID_SIZE],
    pub(crate) owners: [u8; GRID_SIZE],
    pub(crate) takens: [bool; GRID_SIZE],
    pub(crate) valid_moves: ArrayVec<usize, GRID_SIZE>,
    pub(crate) adjacency: [bool; GRID_SIZE],
    pub(crate) turn: usize,
}
        

impl Grid {

    pub fn get_neighbors(location: usize) -> impl Iterator<Item = usize> {
        // i didn't want to fix the wrap around error myself, so 
        // i got an LLM to fix it. hopeful rewrite sooner rather than later
        // ai generated v
        let x = location_to_x(location);
        let y = location_to_y(location);
        let loc = location as isize;

        let offsets = if (y & 1) == 1 {
            ODD_OFFSETS
        } else {
            EVEN_OFFSETS
        };
        let x_isize = X_MAX as isize;

        offsets.into_iter().filter_map(move |off| {
            let n = loc + off;
            if n < 0 || n >= GRID_ISIZE {
                return None;
            }

            // prevent row-wrap on +/-1 and diagonals
            if (off == 1 || off.abs() == x_isize + 1) && x + 1 >= X_MAX {
                return None; // needs a right column
            }
            if (off == -1 || off.abs() == x_isize - 1) && x == 0 {
                return None; // needs a left column
            }

            Some(n as usize)
        })
        // ai generated ^
    }

    pub(crate) fn init() -> Self {
        let values = [0u8; GRID_SIZE];
        let owners = [0u8; GRID_SIZE];
        let turn: usize = 0;

        let mut takens: [bool; GRID_SIZE];

        if HOLE_PROBABILITY > 0.0 {
            takens = [false; GRID_SIZE];
            takens.fill_with(|| random_bool(HOLE_PROBABILITY));
        }
        // consider pregeneration
        else {
            takens = [false; GRID_SIZE];
        }
        let adj = [false; GRID_SIZE];

        let valid_moves: ArrayVec<usize, GRID_SIZE> =
            get_valid_moves_from_takens(&takens).collect();
        Self {
            values: values,
            owners: owners,
            takens: takens,
            valid_moves: valid_moves,
            adjacency: adj,
            turn: turn,
        }
    }

    #[inline]
    pub(crate) fn is_terminal(&self) -> bool {
        self.takens.iter().all(|&tile| tile)
    }

    pub(crate) fn update_neighbors(&mut self, value: u8, owner: u8, location: usize) {
        let neighbors = Self::get_neighbors(location); 
        
        let owners = &mut self.owners;
        let values = &mut self.values;

        for neighbor in neighbors {
            unsafe { // removes bounds checks for speed
                    let o = *owners.get_unchecked(neighbor);
                    let value_ptr = values.get_unchecked_mut(neighbor);
                    let v = *value_ptr;
                    
                    let is_weak: u8 = (v < value) as u8;
                    let is_me = (o == owner) as u8;
                    let is_enemy: u8 = (o != owner) as u8;
                    let is_taken: u8 = (o != 0) as u8;

                    let reinforce = is_me;
                    let capture: u8 = is_weak + is_enemy + is_taken;

                    *value_ptr += reinforce; // 0 if not me, 1 if is
                    // wow mostly branchless prediction !

                    if capture == 3 {
                        *owners.get_unchecked_mut(neighbor) = owner;
                    } // there's probably a clever way to make
                    //   this branchless, but i don't see how
                }
        } // index already verified (if n < 0 || n >= GRID_ISIZE)
    }

    pub(crate) fn update_adjacency(&mut self, location: usize, add: bool) {
        self.adjacency[location] = add
    }
}
