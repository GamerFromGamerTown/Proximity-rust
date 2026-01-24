use arrayvec::ArrayVec;
use colored::{ColoredString, Colorize};
use pyo3::prelude::*;
use rand::Rng;
use rand::{prelude::IndexedRandom, rng};
use rayon::prelude::*;
use std::io::{self};
use std::time::Instant;

use crate::constants::{
    location_to_x, location_to_xy, location_to_y, xy_to_location, COLORS, GRID_SIZE, PLAYER_MOVETYPES,
    PLAYER_NUMBER, RECORD_WINLOSS, SIMULATION_MAX, TWO_POW_32, X_MAX, Y_MAX,
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
#[pyclass]
pub(crate) struct Game {
    grid: Grid,
    players: [Player; PLAYER_NUMBER],
    moves: usize,
    start_time: Instant,
}

impl Game {
    pub(crate) fn new() -> Self {
        let p: [Player; PLAYER_NUMBER] =
            std::array::from_fn(|pnum| Player::init((pnum + 1) as u8, PLAYER_MOVETYPES[pnum]));

        Self {
            grid: Grid::init(),
            players: p, // yay i refactored it :)

            moves: 0,
            start_time: Instant::now(),
        }
    }

    const fn is_hole(taken: bool, owner: u8) -> bool {
        taken && owner == 0
    }

    fn test_eval(&self, playerid: u8, location: usize) -> [bool; 1000000] {
        let max: usize = 1000000;
        let mut moves = [false; 1000000];
        let step: usize = 100;
        let player = self.players[playerid as usize];

        for x in 0..max {
            moves[x] = (self.clone().run_single_rollout(player) == player.id)
        }

        return moves
    }
    fn simulation_loop(&mut self, starting_player: Player) -> u8 {
        let mut current_idx: usize = (starting_player.id as usize) - 1;
        let mut rng = rng();
        loop {
            if self.grid.is_terminal() {
                break;
            }

            current_idx = (current_idx + 1) % PLAYER_NUMBER;

            let current_player = self.players[current_idx];
            self.make_random_move(current_player, &mut rng);
        }

        self.get_winner()
    }

    pub(crate) fn game_loop(&mut self) {
        loop {
            for p in self.players.into_iter() {
                if !self.grid.is_terminal() {
                    self.make_move(p);
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
    pub(crate) fn get_scores(&self) -> [usize; PLAYER_NUMBER] {
        let mut scores: [usize; PLAYER_NUMBER] = [0usize; PLAYER_NUMBER];

        for player_number in 1..=PLAYER_NUMBER {
            let mut pscore: usize = 0;
            for (location, o) in self.grid.owners.iter().enumerate() {
                if *o == player_number as u8 {
                    pscore += self.grid.values[location] as usize;
                }
            }
            scores[player_number - 1] = pscore;
        }
        scores
    }

    fn get_winner(&self) -> u8 {
        let winner = self
            .get_scores()
            .iter()
            .enumerate()
            .max_by_key(|(_, score)| *score)
            .map(|(idx, _)| idx)
            .unwrap();
        (winner + 1) as u8
    }
}

include!("moves.rs");
include!("monte_carlo.rs");
include!("game_py.rs");
include!("display.rs");
