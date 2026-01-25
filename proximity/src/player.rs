use rand::{rng, seq::SliceRandom};

use crate::constants::{NUMBANK_SIZE, PLAYER_MOVETYPES, ROLL_MAX, roll_max};

#[derive(Clone, Copy)]
pub(crate) struct Player {
    pub(crate) id: u8,
    pub(crate) move_type: u8,
    pub(crate) score: usize,
    pub(crate) turn: usize,
    pub(crate) numbank: [u8; NUMBANK_SIZE],
}

impl Player {
    pub(crate) fn init(id: u8, move_type: u8) -> Self {
        let mut rng = rng();
        let mut numarray: [u8; NUMBANK_SIZE] =
            std::array::from_fn(|i: usize| ((i as u8 + 1) % roll_max()) + 1);
        numarray.shuffle(&mut rng);
        let numbank: [u8; NUMBANK_SIZE] = numarray;

        Self {
            id: id,
            move_type: move_type,
            score: 0,
            turn: 0,
            numbank: numbank,
        }
        // probably it's best to use default method, but this'll do for now
    }

    pub(crate) const fn roll(&self) -> u8 {
        // make const
        self.numbank[self.turn]
    }
}
