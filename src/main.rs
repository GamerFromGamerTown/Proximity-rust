// main.rs
#![allow(unused)]

/*
Jan 12: 7600    simulations / second
Jan 14: 9500    simulations / second
Jan 15: 20,000  simulations / second  (wow! goal met already!)
Jan 17: 330,000 simulations / second  (achieved with multithreading)
RUSTFLAGS="-C target-cpu=native" cargo build --release
*/

use colored::{ColoredString, Colorize};
use rand::{prelude::IndexedRandom, random_bool, rng, seq::SliceRandom};
use rayon::{prelude::*, vec}; 
use std::time::{Duration, Instant}; 
use std::io::{self, Read};
use arrayvec::ArrayVec;

// use async_executor::Executor;
// use std::thread;
pub const RECORD_WINLOSS: bool = true;
pub const SIMULATION_MAX: u32 = 5000; // replace this with an error function! see error_function_idea

pub const X_MAX: usize = 10;
pub const Y_MAX: usize = 8;
pub const GRID_SIZE: usize = X_MAX * Y_MAX;
pub const GRID_ISIZE: isize = GRID_SIZE as isize;

pub const PLAYER_NUMBER: usize = 2;
pub const PLAYER_MAX: usize = 5;

pub const NUMBANK_SIZE: usize = usize::div_ceil(GRID_SIZE, PLAYER_NUMBER); // ceiling division!
pub const ROLL_MAX: u8 = 20;
pub const HOLE_PROBABILITY: f64 = 0.0; // 0.0 >= n >= 1

pub const P1MOVETYPE: u8 = 0;
pub const P2MOVETYPE: u8 = 3;
pub const P3MOVETYPE: u8 = 1;
pub const P4MOVETYPE: u8 = 1;

pub const COLORS: [&str; PLAYER_MAX + 1] = ["white", "blue", "red", "green", "magenta", "yellow"];
pub const COLOR_CODES: [&str; PLAYER_MAX + 1] = ["err", "B", "R", "G", "P", "Y"];

pub const EVEN_OFFSETS: [isize; 6] = [
    1,                     // Right
    -1,                    // Left
    X_MAX as isize,        // Bottom
    -(X_MAX as isize),     // Top
    (X_MAX as isize) - 1,  // Bottom-Left
    -(X_MAX as isize) - 1, // Top-Left
];

pub const ODD_OFFSETS: [isize; 6] = [
    1,                     // Right
    -1,                    // Left
    X_MAX as isize,        // Bottom
    -(X_MAX as isize),     // Top
    (X_MAX as isize) + 1,  // Bottom-Right
    -(X_MAX as isize) + 1, // Top-Right
];


fn main() {
    let mut game = Game::initialize();
    game.game_loop();
    println!("{:?}", game.get_scores())
}

// coordinates helpers
pub const fn location_to_x(location: usize) -> usize {
    location % X_MAX
}
pub const fn location_to_y(location: usize) -> usize {
    location / X_MAX
}
pub const fn location_to_xy(location: usize) -> (usize, usize) {
    (location_to_x(location), location_to_y(location))
}
pub const fn xy_to_location(x: usize, y: usize) -> usize {
    (y * X_MAX) + x
}

#[derive(Copy, Clone)]
struct MoveInfo {
    wins: u64,
    total: u64,
    uncertainty: f64,
}

#[derive(Clone, Copy)]
struct Player {
    id: u8,
    move_type: u8,
    score: usize,
    turn: usize,
    numbank: [u8; NUMBANK_SIZE],
}

impl Player {
    fn init(id: u8, move_type: u8) -> Self {
        let mut rng = rng();
        let mut numarray: [u8; NUMBANK_SIZE] =
            std::array::from_fn(|i: usize| ((i as u8 + 1) % ROLL_MAX) + 1);
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

    const fn roll(&self) -> u8 {
        // make const
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
        // ai generated
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
        // ai generated
    }

    fn init() -> Self {
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

        Self {
            values: values,
            owners: owners,
            takens: takens,
            adjacency: adj,
            turn: turn,
        }
    }

    fn is_terminal(&self) -> bool {
        self.takens.iter().all(|&tile| tile)
    }

    fn update_neighbors(&mut self, value: u8, owner: u8, location: usize) {
        // around 5% of execution time, optimize
        let neighbors = Self::get_neighbors(location);

        for neighbor in neighbors {
            let neighbor_owner = self.owners[neighbor];

            if neighbor_owner == 0 {
                continue;
            } else if neighbor_owner == owner {
                self.values[neighbor] += 1;
                // Gameplayer[neighbor_owner].score += 1
            } else if self.values[neighbor] < value {
                self.owners[neighbor] = owner;
                // player[owner].score += value
                // player[neighbor_owner].score -= value
            }
        }
    }

    fn update_adjacency(&mut self, location: usize, add: bool) {
        self.adjacency[location] = add
    }
}

#[derive(Clone)]
struct Game {
    grid: Grid,
    players: [Player; PLAYER_NUMBER],
    total_simulations: usize,
    start_time: Instant,
}

impl Game {
    fn initialize() -> Self {
        Self {
            grid: Grid::init(),
            // FIXME
            players: [Player::init(1, P1MOVETYPE), Player::init(2, P2MOVETYPE)], // halfassed temporary solution
            // TODO
            total_simulations: 0,
            start_time: Instant::now(),
        }   
    }
    // core
    fn add(&mut self, value: u8, owner: u8, location: usize) {
        if self.grid.takens[location] {
            panic!("AAAHHHH")
        }

        self.grid.values[location] = value;
        self.grid.owners[location] = owner;
        self.grid.takens[location] = true;
        self.grid.update_neighbors(value, owner, location);
        self.players[(owner - 1) as usize].turn += 1;
        self.players[(owner - 1) as usize].score += value as usize;
        self.grid.turn += 1;
    }

    fn get_valid_moves(&self) -> impl Iterator<Item = usize> {
        // make this an iterator and we'll all be happy
        self.grid
            .takens
            .iter()
            .enumerate()
            .filter(|(_, tile)| !*tile) // only include true tiles
            .map(|(idx, _)| idx) // ignore the tile bool itself
    }

    const fn is_hole(taken: bool, owner: u8) -> bool {
        taken && owner == 0
    }

    fn display(&self, show_sim_sec: bool) {
        let mut board: String = String::from("");
        for idx in 0..GRID_SIZE {
            let mut tile = ColoredString::from("");
            let padding = "  ";
            let taken = self.grid.takens[idx];
            let owner = self.grid.owners[idx];
            let value = self.grid.values[idx];
            let mut v = self.grid.values[idx].to_string();

            let is_hole = Game::is_hole(taken, owner);

            if value < 10 {
                v = "0".to_owned() + &v // 4 -> 04
            }
            v = "P".to_owned() + &v; // p is placeholder for player color

            if location_to_x(idx) == 0 {
                board.push_str("\n");
                if location_to_y(idx) % 2 == 1 {
                    board.push_str(padding)
                }
            }

            if owner > 0 {
                tile = Into::into(v)
            } else if is_hole {
                tile = Into::into(" X ")
            } else if !is_hole {
                tile = Into::into(" · ")
            }

            tile = tile.color(COLORS[owner as usize]);
            board.push_str(&tile.to_string());
            board.push_str(&padding.to_string());
        }
        println!("{}", board);

        if show_sim_sec {
            let elapsed = self.start_time.elapsed().as_secs_f64();

            println!("About {} simulations per second.", 
            (self.total_simulations as f64 / elapsed).round())}
    }

    fn simulation_loop(&mut self, starting_player: Player) -> u8 {
        let mut current_idx: usize = (starting_player.id as usize) - 1;
        loop {
            if self.grid.is_terminal() {
                break;
            }
            // advance to next player
            current_idx = (current_idx + 1) % PLAYER_NUMBER;

            let current_player = self.players[current_idx];
            self.make_random_move(current_player);
        }

        self.get_winner()
    }

    fn game_loop(&mut self) {
        loop {
            for p in self.players.into_iter() {
                if !self.grid.is_terminal() {
                    self.make_move(p);
                } else {
                    break;
                }
            }
            if self.grid.is_terminal() {break}
        }
    }

    // scoring / winning
    fn get_scores(&self) -> [usize; PLAYER_NUMBER] {
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
        let winner = self
            .get_scores()
            .iter()
            .enumerate()
            .max_by_key(|(_, score)| *score)
            .map(|(idx, _)| idx)
            .unwrap();
        (winner + 1) as u8
    }

    // move types
    fn make_random_move(&mut self, player: Player) {
        let mut rng = rng();
        
        let mut moves: ArrayVec<usize, GRID_SIZE> = ArrayVec::new();
        for m in self.get_valid_moves() {
            moves.push(m);
        }

        let chosen_move: usize = *moves.choose(&mut rng).expect("Game is not terminal.");

        self.add(player.roll(), player.id, chosen_move);
    }

    fn make_greedy_move(&mut self, player: Player) {
        let mut moves: ArrayVec<usize, GRID_SIZE> = ArrayVec::new();
        
        for m in self.get_valid_moves() {
            moves.push(m)
        }

        let mut best_move: usize = moves[0];
        let mut best_score: u8 = 0;

        for move_choice in moves.iter() {
            let current_score = self.get_score_from_move(*move_choice, player.id, player.roll());
            if current_score > best_score as u8 {
                best_move = *move_choice;
                best_score = current_score;
            }
        }

        self.add(player.roll(), player.id, best_move as usize);
        self.display(false);
    }

    fn make_monte_carlo_flat_move(&mut self, player: Player) {
        self.start_time = Instant::now();
        let moves: Vec<usize> = self.get_valid_moves().collect();

        let mut moves_info: [MoveInfo; GRID_SIZE] = [MoveInfo {
            wins: 0,
            total: 100000000000000,
            uncertainty: 0.0,
        }; GRID_SIZE];
        

        let ptr = moves_info.as_mut_ptr() as usize;
        moves.par_iter().for_each(|tile| {
            let (current_wins, current_total) = self.evaluate(player, *tile);

            unsafe { // evil multithreading
                if *tile >= GRID_SIZE {panic!()}
                let moveinfo_ptr = (ptr as *mut MoveInfo).add(*tile);
                (*moveinfo_ptr).wins = current_wins as u64;
                (*moveinfo_ptr).total = current_total as u64;
            }  // it's safe because each index is exclusive
        });
        self.total_simulations += (SIMULATION_MAX as usize * moves.len()) as usize;

        let best_move = 
        moves_info
            .iter()
            .enumerate()
            .max_by_key(|&item| (item.1.wins) * (65536) / (item.1.total))
            // ^^ janky ass solution to add more precision to integer division,
            // since float division isn't supported.
            .map(|(idx, _)| idx);

        self.add(player.roll(), player.id, best_move.unwrap());
        self.display(true);
    }

    fn make_human_move(&mut self, player: Player) {
        self.display(false);
        loop {
        println!("Please choose an X and Y to place the tile. Add a comma afterwards, too. Example: 5,3, \n Your roll is {}", player.roll());

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let separated_input: Vec<&str> = input.split(",").collect();

            let xy: Vec<usize> = separated_input.iter()
            .filter_map(|v| v.parse::<usize>().ok())
            .collect::<Vec<usize>>();

            println!("{:?}", xy);
            if xy.len() != 2 {
                println!("Invalid input. Please try again.");
                continue
            }
            
            let x: usize = xy[0];
            let y: usize = xy[1];
            let location = xy_to_location(x, y);

            if location_to_x(location) > X_MAX || location_to_y(location) > Y_MAX {
                println!("Location out of bounds! Try again!");
                continue;
            }

            if self.grid.takens[location] {
                println!("Tile already taken! Try again.");
                continue;
            }
            
            self.add(player.roll(), player.id, location);
            break
        }
        
    }

    // simulation stuff
    fn run_single_rollout(&mut self, player: Player) -> u8 {
        self.simulation_loop(player)
    } // returns the winner from one randomly-played game

    fn evaluate(&self, player: Player, location: usize) -> (u32, u32) {
        // chokepoint for parallelization
        let mut win_count: u32 = 0;
        let mut game_count: u32 = 0;
        let mut copy = self.clone();

        copy.add(player.roll(), player.id, location);

        loop {
            game_count += 1;
            if game_count >= SIMULATION_MAX {
                break;
            }

            if copy.clone().run_single_rollout(player.clone()) == player.id {
                win_count += 1;
                if RECORD_WINLOSS {}
                // break
            }
        }
        (win_count, game_count)
    }

    // misc
    fn make_move(&mut self, player: Player) {
        if player.move_type == 0 {
            self.make_human_move(player);
        } else if player.move_type == 1 {
            self.make_random_move(player);
        } else if player.move_type == 2 {
            self.make_greedy_move(player)
        } else if player.move_type == 3 {
            self.make_monte_carlo_flat_move(player)
        }
    }

    fn get_score_from_move(&self, location: usize, owner: u8, value: u8) -> u8 {
        let neighbors = Grid::get_neighbors(location);
        let mut score: u8 = 0;

        for neighbor in neighbors {
            let neighbor_owner = self.grid.owners[neighbor];

            if neighbor_owner == 0 {
                continue;
            } else if neighbor_owner == owner {
                score += 1
            } else if self.grid.values[neighbor] < value {
                score += value
            }
        }
        score
    }
}
