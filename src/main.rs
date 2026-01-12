// main.rs
#![allow(unused)]
// get rid of this during compile time, it's just annoying during prototyping

use rand::{random_bool, rng, seq::SliceRandom, prelude::IndexedRandom};
use colored::Colorize;
use std::iter::zip;

//#region
pub const X_MAX: usize      = 10;
pub const Y_MAX: usize      = 8;
pub const GRID_SIZE: usize  = X_MAX * Y_MAX;
pub const PLAYER_NUMBER: usize = 2;
pub const NUMBANK_SIZE:  usize = usize::div_ceil(GRID_SIZE, PLAYER_NUMBER); // ceiling division!
pub const ROLL_MAX: u8 = 20;
pub const HOLE_PROBABILITY: f64 = 0.0; // 0.0 >= n >= 1

pub const P1MOVETYPE: u8 = 1;
pub const P2MOVETYPE: u8 = 2;
pub const P3MOVETYPE: u8 = 1;
pub const P4MOVETYPE: u8 = 1;

// 

//#endregion
/* maybe move the above constants into the struct for the grid */

fn main() {
    let mut game = Game::initialize();
    game.game_loop();
}
pub const fn get_x(location: usize) -> usize {
    location % X_MAX
}
pub const fn get_y(location: usize) -> usize {
    location / X_MAX // this is floor division, apparently
}

#[derive(Clone, Copy)]
struct Player {
    id: u8,
    move_type: u8,
    score: usize,
    sum_of_rolls: usize,
    turn: usize, // used to point at numbank, instead of having to pop numbank every time
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
            sum_of_rolls: 0,
            turn: 0,
            numbank: numbank
        }
        // probably it's best to use default method, but this'll do
    }

    const fn roll(&self) -> u8 {
        self.numbank[self.turn]
    }
}

#[derive(Clone)]
struct Grid {
    values: [u8; GRID_SIZE],
    owners: [u8; GRID_SIZE],
    takens: [bool; GRID_SIZE],
    adjacency: [bool; GRID_SIZE],

    // tile_count: usize,
    turn: usize,
}

impl Grid {
    fn get_neighbors(&self, location: usize) -> Vec<usize>{
        // really verbose and probably not idiotomic 
        let is_odd = (get_y(location) % 2 == 1);
        let mut neighbors: Vec<usize> = vec![];

        if (location + 1 < GRID_SIZE) && ((location + 1) % X_MAX != 0) && self.takens[location + 1] {
            neighbors.push(location + 1)}       // right neighbor
        
        if (location > 0) && (location % X_MAX != 0) && self.takens[location - 1]{
            neighbors.push(location - 1)}       // left neighbor

        if location + X_MAX < GRID_SIZE && self.takens[location + X_MAX] {
            neighbors.push(location + X_MAX);   // bottom neighbor (1)
        }
        
        if location >= X_MAX && self.takens[location - X_MAX]  {
            neighbors.push(location - X_MAX);   // top neighbor (1)
        }

        // maybe just add 1 if odd and subtract 1 if not, making the + or - 1 depend on is_odd
        if is_odd {
            if location >= X_MAX && ((location + 1) % X_MAX != 0) && self.takens[location - X_MAX + 1] {
                neighbors.push(location - X_MAX + 1)
            }
            if location + X_MAX + 1 < GRID_SIZE && ((location + 1) % X_MAX != 0) && self.takens[location + X_MAX + 1] {
                neighbors.push(location + X_MAX + 1)
            }
        }
        
        else {
            if location >= X_MAX + 1 && (location % X_MAX != 0) && self.takens[location - X_MAX - 1] {
                neighbors.push(location - X_MAX - 1)
            }
            if location + X_MAX - 1 < GRID_SIZE && (location % X_MAX != 0) && self.takens[location + X_MAX - 1] {
                neighbors.push(location + X_MAX - 1)
            }
        }

        neighbors
    }

    fn init() -> Self {
        let values = [0u8; GRID_SIZE];
        let owners = [0u8; GRID_SIZE];
        let turn: usize = 0;
        
        let mut takens: [bool; GRID_SIZE];
        
        if HOLE_PROBABILITY > 0.0 {
            takens = [false; GRID_SIZE]; // consider switching vec to arc[<t>]; look into it at least
            takens.fill_with(|| {
                random_bool(HOLE_PROBABILITY)
            });
        }

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
        let neighbors = self.get_neighbors(location);

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

    // fn clone(&self) -> Self {
    //     self
    // }

    fn display(&self) {
        let mut board: String = String::from("");
        for idx in 0..GRID_SIZE{
            let mut tile = String::from("");
            let padding = "  ";
            let taken = self.grid.takens[idx];
            let owner = self.grid.owners[idx];
            let value = self.grid.values[idx];
            let mut v = self.grid.values[idx].to_string();

            let is_hole =Game::is_hole(taken, owner);

            if value < 10 { // 4 -> 04
                v = "0".to_owned() + &v
            }
            v = "P".to_owned() + &v; // p is placeholder for balancing

            if get_x(idx) == 0 {
                board.push_str("\n");
                if get_y(idx) % 2 == 1 {
                    board.push_str(padding)
                }
            }

            if owner > 0 {tile = v}
            if owner == 1 {     tile = tile.blue() .to_string();}
            else if owner == 2 {tile = tile.red()  .to_string();}
            else if owner == 3 {tile = tile.green().to_string();}
            else if is_hole  {  tile = " X ".to_string()}
            else if !is_hole {  tile = " · ".to_string()}

            board.push_str(&tile);
            board.push_str(padding);
        }
        println!("{}", board)
    }

    pub fn game_loop(&mut self){
        while !self.grid.is_terminal() {
            for p in self.players.into_iter() { // ISSUE--even if game is not terminal, it can be in the next 2 moves, 
                if !self.grid.is_terminal(){            // which is not accounted for in the for loop. I added a secondary check,
                self.make_move(p)}              // but it is superfluous and should be replaced later
            }
        }

        self.display();

        let scores = self.get_scores();
        for (p, s) in zip(self.players.iter_mut(), scores){
            p.score = s
        }

        println!("Player1:{}, Player2:{}", self.players[0].score, self.players[1].score)
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

        }
        scores
    }

    fn get_valid_moves(&self) -> Vec<usize>{
        // self.grid.takens.iter()
        // .filter(|&tile| !tile);
        let mut valid_moves: Vec<usize> = Vec::with_capacity(GRID_SIZE);
        for (idx, tile) in self.grid.takens.iter().enumerate(){
            if *tile == false {
                valid_moves.push(idx); 
            }
        }
        valid_moves
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
    }

    fn make_random_move(&mut self, player: Player) {
        let mut rng = rng();
        let moves: Vec::<usize>  = self.get_valid_moves();
        let chosen_move: usize = *moves.choose(&mut rng).expect("Game is not terminal.");
        
        self.add(player.roll(), player.id, chosen_move);
        self.display();
    }

    fn get_score_from_move(&self, location: usize, owner: u8, value: u8) -> u8 {
        let neighbors = &self.grid.get_neighbors(location);
        let mut score: u8 = 0;

        for neighbor in neighbors {
            let neighbor_owner = self.grid.owners[*neighbor];
    
            if neighbor_owner == 0 {continue}
            
            else if neighbor_owner == owner {
                score += 1
            }

            else if self.grid.values[*neighbor] < value {
                score += value
            } 
        }
        score
    }

    fn make_greedy_move(&mut self, player: Player) {
        // BAD! IT TRIES TO TAKE ALREADY TAKEN TILES
        let mut rng = rng();
        let moves: Vec::<usize>  = self.get_valid_moves();
        let mut best_move = moves[0];

        for move_choice in moves.iter(){
            let current_score = self.get_score_from_move(*move_choice, player.id, player.roll());
            if current_score > best_move as u8 {
                best_move = *move_choice}
        }
        
        for m in self.get_valid_moves().iter(){
            println!("{},{},{}", get_x(*m as usize), get_y(*m as usize), self.grid.takens[*m])
        }

        println!("Chose {},{} ({})", get_x(best_move as usize), get_y(best_move as usize), best_move);
        
        self.add(player.roll(), player.id, best_move as usize);
        self.display();
    }

}
