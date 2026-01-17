// main.rs
#![allow(unused)]

/*
Jan 12: 7600    simulations / second 
Jan 14: 9500    simulations / second
Jan 15: 20,000  simulations / second  (wow! goal met already!)
RUSTFLAGS="-C target-cpu=native" cargo build --release
*/ 

use rand::{random_bool, rng, seq::SliceRandom, prelude::IndexedRandom};
use colored::{ColoredString, Colorize};
use rayon::prelude::*;


pub const X_MAX: usize      = 10;
pub const Y_MAX: usize      = 8;
pub const GRID_SIZE: usize  = X_MAX * Y_MAX;
pub const GRID_ISIZE: isize = GRID_SIZE as isize;

pub const PLAYER_NUMBER: usize = 2;
pub const PLAYER_MAX:    usize = 5;

pub const NUMBANK_SIZE:  usize = usize::div_ceil(GRID_SIZE, PLAYER_NUMBER); // ceiling division!
pub const ROLL_MAX: u8 = 20;
pub const HOLE_PROBABILITY: f64 = 0.0; // 0.0 >= n >= 1

pub const P1MOVETYPE: u8 = 2;
pub const P2MOVETYPE: u8 = 3;
pub const P3MOVETYPE: u8 = 1;
pub const P4MOVETYPE: u8 = 1;

pub const COLORS:      [&str; PLAYER_MAX+1] = ["white", "blue", "red", "green", "magenta", "yellow"];
pub const COLOR_CODES: [&str; PLAYER_MAX+1] = ["err",   "B",    "R",   "G",     "P",       "Y"];


// in an odd-r grid, these are the offsets away from a tile
pub const EVEN_OFFSETS: [isize; 6] = [
    1,                         // Right
    -1,                        // Left
    X_MAX as isize,            // Bottom
    -(X_MAX as isize),         // Top
    (X_MAX as isize) - 1,      // Bottom-Left
    -(X_MAX as isize) - 1,     // Top-Left
];

pub const ODD_OFFSETS: [isize; 6] = [
    1,                         // Right
    -1,                        // Left
    X_MAX as isize,            // Bottom
    -(X_MAX as isize),         // Top
    (X_MAX as isize) + 1,      // Bottom-Right
    -(X_MAX as isize) + 1,     // Top-Right
];


fn main() {
    let mut game = Game::initialize();
    game.game_loop();
}

pub const fn get_x(location: usize) -> usize {
    location % X_MAX
}
pub const fn get_y(location: usize) -> usize {
    location / X_MAX 
}

pub const fn get_xy(location: usize) -> (usize, usize) {
    (get_x(location), get_y(location))
}


#[derive(Clone, Copy)]
struct Player {
    id: u8,
    move_type: u8,
    score: usize,
    turn: usize, 
    numbank: [u8; NUMBANK_SIZE]
}

impl Player {
    fn init(id: u8, move_type: u8) -> Self {
        let mut rng = rng();
        let mut numarray: [u8; NUMBANK_SIZE] = std::array::from_fn(|i: usize| ((i as u8 + 1) % ROLL_MAX)+1);
        numarray.shuffle(&mut rng);
        let numbank: [u8; NUMBANK_SIZE] = numarray;
        
        Self {
            id: id,
            move_type: move_type,
            score: 0,
            turn: 0,
            numbank: numbank
        }
        // probably it's best to use default method, but this'll do for now
    }

    const fn roll(&self) -> u8 { // make const
        self.numbank[self.turn]        
    }
}

#[derive(Clone)]
struct Grid {
    values: [u8; GRID_SIZE],
    owners: [u8; GRID_SIZE],
    takens: [bool; GRID_SIZE],
    adjacency: [bool; GRID_SIZE],
    turn: usize,
}

impl Grid {
    pub fn get_neighbors(location: usize) -> impl Iterator<Item = usize> {
        // takes 40% of runtimes
        // OPTIMIZE, sometimes speed > readability/functional code

        let loc = location as isize;
        let is_odd = get_y(location) % 2 == 1;
        let offsets: [isize; 6] = if is_odd {ODD_OFFSETS} else {EVEN_OFFSETS};
        
        offsets
            .into_iter()
            .map(move |x: isize| loc+x) // thanks compiler for the awesome hint but i still dunno what move did
            .filter(|x: &isize | GRID_ISIZE > *x && *x >= 0)
            .map(|x| x as usize)
}

    fn init() -> Self {
        let values = [0u8; GRID_SIZE];
        let owners = [0u8; GRID_SIZE];
        let turn: usize = 0;
        
        let mut takens: [bool; GRID_SIZE];
        
        if HOLE_PROBABILITY > 0.0 {
            takens = [false; GRID_SIZE];
            takens.fill_with(|| {
                random_bool(HOLE_PROBABILITY)
            });
        } // consider pregeneration

        else {
            takens  = [false; GRID_SIZE];
        }

        let adj = [false; GRID_SIZE];

        Self {
            values: values,
            owners: owners,
            takens: takens,
            adjacency: adj,
            turn  : turn,
        }
    }

    fn is_terminal(&mut self) -> bool {
        self.takens.iter().all(|&tile| tile)
    }

    fn update_neighbors(&mut self, value: u8, owner: u8, location: usize) {
        // around 5% of execution time, optimize 
        let neighbors = Self::get_neighbors(location);

        for neighbor in neighbors {
            let neighbor_owner = self.owners[neighbor];
    
            if neighbor_owner == 0 {continue}
            
            else if neighbor_owner == owner {
                self.values[neighbor] += 1;                
                // Gameplayer[neighbor_owner].score += 1
            }

            else if self.values[neighbor] < value {
                self.owners[neighbor] = owner;
                // player[owner].score += value
                // player[neighbor_owner].score -= value
            } 
        }
    }

    fn update_adjacency(&mut self, location: usize, add: bool){
        self.adjacency[location] = add
    }
}

#[derive(Clone)]
struct Game {
    grid: Grid,
    players: [Player; PLAYER_NUMBER]
}

impl Game {
    fn add(&mut self, value: u8, owner: u8, location: usize){
        if self.grid.takens[location] {
            panic!("AAAHHHH")
        }

        self.grid.values[location] = value;
        self.grid.owners[location] = owner;
        self.grid.takens[location] = true;
        self.grid.update_neighbors(value, owner, location);
        self.players[(owner - 1) as usize].turn  += 1;
        self.players[(owner - 1) as usize].score += value as usize;
        self.grid.turn += 1;
    }

    fn display(&self) {
        let mut board: String    = String::from("");
        for idx in 0..GRID_SIZE{
            let mut tile = ColoredString::from("");
            let padding    = "  ";
            let taken      = self.grid.takens[idx];
            let owner        = self.grid.owners[idx];
            let value        = self.grid.values[idx];
            let mut v    = self.grid.values[idx].to_string();

            let is_hole    = Game::is_hole(taken, owner);

            if value < 10 { // 4 -> 04
                v = "0".to_owned() + &v
            }
            v = "P".to_owned() + &v; // p is placeholder for player color

            if get_x(idx) == 0 {
                board.push_str("\n");
                if get_y(idx) % 2 == 1 {
                    board.push_str(padding)
                }
            }
            
            if owner > 0        {tile = Into::into(v)}
            else if is_hole     {tile = Into::into(" X ")}
            else if !is_hole    {tile = Into::into(" · ")}

            tile = tile.color(COLORS[owner as usize]);
            board.push_str(&tile.to_string());
            board.push_str(&padding.to_string());
        }
        println!("{}", board)
    }

    fn simulation_loop(&mut self, player: Player) -> u8 {
        let mut idx: usize = player.id as usize;
        // REMEMBER TO SHUFFLE NUMBANK
        loop {
            if !self.grid.is_terminal(){
                self.make_random_move(self.players[(idx+1) % PLAYER_NUMBER]);
                idx += 1;
            }
            else {
                break
            }
        }
        return self.get_winner()

    }

    fn game_loop(&mut self) { 
        loop {
            for p in self.players.into_iter() { 
                if !self.grid.is_terminal(){
                     self.make_move(p);
                }          
                else {break}
            }
        }
    }


    fn get_scores(&self) -> [usize; PLAYER_NUMBER] {
        let mut scores: [usize; PLAYER_NUMBER] = [0usize; PLAYER_NUMBER];

        for player_number in 1..=PLAYER_NUMBER {
            let mut pscore: usize = 0;
            for (location, o) in self.grid.owners.iter().enumerate(){
                if *o == player_number as u8 {
                    pscore += self.grid.values[location] as usize;
                }
            }
            scores[player_number-1] = pscore;
        } scores


        /*
        We need to:
            Approach 1
            First, get an iter of self.grid.owners
            Then, enumerate
            Then, split it into a list of vecs with each owner, and only care about locations
            Then, get the values of all of these locations, in the same vec.
            Then, get the sum of each of these vecs.
            Finally, return.*/
        // self.grid.values
        // .iter()
        // .enumerate()
        // .zip()
}

    fn get_winner(&self) -> u8 {
        let winner = self.get_scores()
        .iter()
        .enumerate()
        .max_by_key(|(_, score)| *score)
        .map(|(idx, _)| idx)
        .unwrap();
        (winner + 1) as u8
    }
    fn get_valid_moves(&self) -> Vec<usize>{
        self.grid.takens
        .iter()
        .enumerate()
        .filter(|(_, tile)| !*tile) // only include true tiles
        .map(|(idx, _)| idx)                     // ignore the tile bool itself
        .collect()                                                                 // return the array of indices
    }

    fn initialize() -> Self {
        Self {
            grid: Grid::init(),
            // FIXME 
            players: [Player::init(1, P1MOVETYPE), Player::init(2, P2MOVETYPE)], // halfassed temporary solution
            // TODO
        }
    }

    pub fn is_hole(taken: bool, owner: u8) -> bool {
        taken && owner == 0
    }

    fn make_move(&mut self, player: Player){
        if player.move_type == 0 {      panic!("Add make_human_move.") }
        else if player.move_type == 1 { self.make_random_move(player); }
        else if player.move_type == 2 { self.make_greedy_move(player)}
        else if player.move_type == 3 { self.make_monte_carlo_flat_move(player)}
    }

    fn make_random_move(&mut self, player: Player) {
        let mut rng = rng();
        let moves: Vec::<usize>  = self.get_valid_moves();
        let chosen_move: usize = *moves.choose(&mut rng).expect("Game is not terminal.");
        
        self.add(player.roll(), player.id, chosen_move);
    }

    fn get_score_from_move(&self, location: usize, owner: u8, value: u8) -> u8 {

        let neighbors = Grid::get_neighbors(location);
        let mut score: u8 = 0;

        for neighbor in neighbors {
            let neighbor_owner = self.grid.owners[neighbor];
    
            if neighbor_owner == 0 {continue}
            
            else if neighbor_owner == owner {
                score += 1
            }

            else if self.grid.values[neighbor] < value {
                score += value
            } 
        }
        score
    }

    fn make_greedy_move(&mut self, player: Player) {
        let moves: Vec::<usize>  = self.get_valid_moves();
        let mut best_move = moves[0];
        let mut best_score: usize = 0;

        for move_choice in moves.iter(){
            let current_score = self.get_score_from_move(*move_choice, player.id, player.roll());
            if current_score > best_score as u8 {
                best_move = *move_choice}
        }        

        self.add(player.roll(), player.id, best_move as usize);
        self.display();
    }
    
    fn run_single_rollout(&mut self, player: Player) -> u8 {
        self.simulation_loop(player)} // always returns 1s for some reason


    fn evaluate(&mut self, player: Player, location: usize) -> (u32, u32) { // chokepoint for parallelization
        const SIMULATION_MAX: u32 = 5000; // replace this with an error function! see error_function_idea
        let mut win_count:  u32 = 0;
        let mut game_count: u32 = 0;
        let mut copy = self.clone();
        
        copy.add(player.roll(), player.id, location);
        
        loop {
            game_count += 1; 
            if game_count >= SIMULATION_MAX {break}
            if copy.clone().run_single_rollout(player.clone()) == player.id {win_count += 1}
        }
        println!("{}", player.id);
        (win_count, game_count)
    }
    fn make_monte_carlo_flat_move(&mut self, player: Player) {
        let mut best_move: (f32, usize) = (0.0, GRID_SIZE+1); // score (wins/total), location

        for tile in self.get_valid_moves() {
            let (current_wins, current_total) = self.evaluate(player, tile);
            let current_move_score = current_wins as f32 / current_total as f32;
            if best_move.0 < current_move_score {
                best_move = (current_move_score, tile)
            }
            println!("Best move is {:?}, looking at {:?}", get_xy(best_move.1), get_xy(tile))
        }
        self.add(player.roll(), player.id, best_move.1);
        self.display();
    }

}
