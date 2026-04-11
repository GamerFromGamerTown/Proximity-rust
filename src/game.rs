use arrayvec::ArrayVec;
use colored::{ColoredString, Colorize};
use pyo3::{pymethods, pyclass};
use rand::{Rng, rng, SeedableRng, prelude::IndexedRandom};
use std::io::{self};
use std::time::Instant;
// use memchr::memchr;

use crate::constants::{
    ADD_TILE_CHECK, COLORS, GRID_SIZE, PLAYER_NUMBER, ROLL_MAX, TWO_POW_32, X_MAX, Y_MAX, location_to_x, location_to_y, player_movetypes, xy_to_location
};

use crate::grid::Grid;
use crate::player::Player;

#[derive(Copy, Clone)]
struct MoveInfo {
    wins: u64,
    total: u64,
    uncertainty: u32,
}

#[derive(Clone)]
// #[pyclass]
pub(crate) struct Game {
    grid: Grid,
    players: [Player; PLAYER_NUMBER],
    moves: usize,
    start_time: Instant,
    absolute_turn: usize,
}

impl Game {
    pub(crate) fn new() -> Self {
        let p: [Player; PLAYER_NUMBER] =
            std::array::from_fn(|pnum| 
                Player::init((pnum + 1) as u8, player_movetypes()[pnum]));

        Self {
            grid: Grid::init(),
            players: p, // yay i refactored it :)

            moves: 0,
            start_time: Instant::now(),
            absolute_turn: 0
        }
    }

    const fn is_hole(taken: bool, owner: u8) -> bool {
        taken && owner == 0
    }

    fn simulation_loop(&mut self, starting_player: Player, mut rng: &mut SmallRng) -> u8 {
        let mut current_idx: usize = (starting_player.id as usize) - 1;
        
        let mut rng2 = rand::rng();
        loop {
            if self.grid.is_terminal() {
                break;
            }

            current_idx = (current_idx + 1) % PLAYER_NUMBER;

            let current_player = self.players[current_idx];
            self.make_random_move(current_player, &mut rng2);
        }

        self.get_winner()
    }

    pub(crate) fn game_loop(&mut self) {
        let mut rng: SmallRng = SmallRng::from_rng(&mut rand::rng());        
        loop {
            for p in self.players.into_iter() {
                if !self.grid.is_terminal() {
                    self.make_move(p, &mut rng);
                } else {
                    break;
                }
            }
            if self.grid.is_terminal() {
                break;
            }
        }
    }

    // scoring / winning
    pub(crate) fn get_scores(&self, from_score: bool) -> [usize; PLAYER_NUMBER] {
        if from_score {
            std::array::from_fn(|i| self.players[i].score)
        }
        else {
            self.grid.values
            .iter()
            .zip(self.grid.owners.iter());
            
            [1, 1]

        }
    }

    fn get_winner(&self) -> u8 {
        let winner = self
            .get_scores(true)
            .iter()
            .enumerate()
            .max_by_key(|(_, score)| *score)
            .map(|(idx, _)| idx)
            .unwrap();
        (winner + 1) as u8
    }

    // #[inline(never)] // TEMP BENCHMARK FIXME
    fn remove_valid_tile(&mut self, location: usize) {
        let idx = self.grid
            .valid_moves
            .iter()
            .position(|&x | x == location) // unlikely
            .expect("valid moves must have position");
        self.grid.valid_moves.swap_remove(idx); 
    }    // the real binary search was the friends we made along the way

    fn add(&mut self, value: u8, owner: u8, location: usize) {
        if ADD_TILE_CHECK && self.grid.takens[location] {
                panic!("Chose taken tile.");
            }
        self.grid.values[location] = value;
        self.grid.owners[location] = owner;
        self.grid.takens[location] = true;

        self.remove_valid_tile(location);

        self.grid.update_neighbors(value, owner, location, &mut self.players);
        // another third is in update_neighbors

        self.players[(owner - 1) as usize].turn += 1;
        self.players[(owner - 1) as usize].score += value as usize;
        self.grid.turn += 1;
    }

}

include!("moves.rs");
include!("monte_carlo.rs");
include!("display.rs");
