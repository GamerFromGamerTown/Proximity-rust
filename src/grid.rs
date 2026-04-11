use arrayvec::ArrayVec;
use rand::random_bool;

use crate::{constants::{
    EVEN_OFFSETS, GRID_ISIZE, GRID_SIZE, ODD_OFFSETS, PLAYER_MAX, PLAYER_NUMBER, X_MAX, get_valid_moves_from_takens, hole_probability, location_to_x, location_to_y
}, player::Player};


#[derive(Clone)]
pub(crate) struct Grid {
    pub(crate) values: [u8; GRID_SIZE],
    pub(crate) owners: [u8; GRID_SIZE],
    pub(crate) takens: [bool; GRID_SIZE],
    pub(crate) valid_moves: ArrayVec<usize, GRID_SIZE>,
    pub(crate) valid_moves_indices: ArrayVec<usize, GRID_SIZE>,
    pub(crate) adjacency: [bool; GRID_SIZE],
    pub(crate) turn: usize,
}
        

impl Grid {
    // #[inline(never)] // TEMP BENCHMARK FIXME
    pub fn get_neighbors(location: usize) -> impl Iterator<Item = usize> {
        // i didn't want to fix the wrap around error myself, so 
        // i got an LLM to fix it. hopeful rewrite sooner rather than later
        // LLM generated v
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
        // LLM generated ^
    }

    pub(crate) fn init() -> Self {
        let values = [0u8; GRID_SIZE];
        let owners = [0u8; GRID_SIZE];
        let turn: usize = 0;

        let mut takens: [bool; GRID_SIZE];

        if hole_probability() > 0.0 {
            takens = [false; GRID_SIZE];
            takens.fill_with(|| random_bool(hole_probability()));
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
            valid_moves: valid_moves.clone(),
            valid_moves_indices: valid_moves,
            adjacency: adj,
            turn: turn,
        }
    }

    #[inline]
    pub(crate) fn is_terminal(&self) -> bool {
        self.valid_moves.is_empty()
    }

    // #[inline(never)] // TEMP BENCHMARK FIXME
    pub(crate) fn update_neighbors(&mut self, value: u8, owner: u8, location: usize, players: &mut [Player; PLAYER_NUMBER]) {
        let neighbors = Self::get_neighbors(location); 
        let owners = &mut self.owners;
        let values = &mut self.values;

        let mut score_deltas: [i16; PLAYER_NUMBER] = [0i16; PLAYER_NUMBER];
        // stores an array with the score changes for each player after updating
        // faster to update all scores at once than to do it every time

        // Pre-calculate your own score index and pointer
        let my_idx = (owner - 1) as usize;

        // We assume `neighbors` is a valid iterator of usize indices
        for neighbor in neighbors {
            unsafe {
                let nb_val_ptr = values.get_unchecked_mut(neighbor);
                let nb_own_ptr = owners.get_unchecked_mut(neighbor);

                let old_val = *nb_val_ptr;
                let old_own = *nb_own_ptr;

                let is_my_color = old_own == owner;
                let is_capture = (!is_my_color) & (old_own != 0) & (old_val < value);
                let capture_mask = -(is_capture as i8) as u8; // 0xFF if capture, else 0x00

                *nb_val_ptr += is_my_color as u8;
                *nb_own_ptr = old_own ^ ((old_own ^ owner) & capture_mask);

                let val_change = (old_val & capture_mask) as i16;      // captured value or 0
                let attacker_delta = (is_my_color as i16) + val_change; // +1 reinforce, +captured on capture
                let defender_delta = -val_change;                       // -captured on capture, else 0

                let def_idx = if is_capture { (old_own - 1) as usize } else { my_idx };

                players[my_idx].score = (players[my_idx].score as i16 + attacker_delta) as usize;
                players[def_idx].score = (players[def_idx].score as i16 + defender_delta) as usize;
            }
        } // index already verified (if n < 0 || n >= GRID_ISIZE)

    }

    pub(crate) const fn update_adjacency(&mut self, location: usize, add: bool) {
        self.adjacency[location] = add
    }
}
