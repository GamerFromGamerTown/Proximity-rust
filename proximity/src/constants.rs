use arrayvec::ArrayVec;
use std::sync::OnceLock;

// ============== COMPILE-TIME CONSTANTS (unchanged for backward compatibility) ==============
pub const RECORD_WINLOSS: bool = true;
pub const SIMULATION_MAX: u32 = 5000;
pub const TWO_POW_32: u64 = 4294967296;
pub const X_MAX: usize = 10;
pub const Y_MAX: usize = 8;
pub const GRID_SIZE: usize = X_MAX * Y_MAX;
pub const GRID_ISIZE: isize = GRID_SIZE as isize;

pub const PLAYER_NUMBER: usize = 2;
pub const PLAYER_MAX: usize = 5;

pub const NUMBANK_SIZE: usize = usize::div_ceil(GRID_SIZE, PLAYER_NUMBER);
pub const ROLL_MAX: u8 = 20;
pub const HOLE_PROBABILITY: f64 = 0.0;

pub const P1MOVETYPE: u8 = 3;
pub const P2MOVETYPE: u8 = 3;
pub const P3MOVETYPE: u8 = 1;
pub const P4MOVETYPE: u8 = 1;
pub const P5MOVETYPE: u8 = 1;

pub const PLAYER_MOVETYPES: [u8; PLAYER_MAX] =
    [P1MOVETYPE, P2MOVETYPE, P3MOVETYPE, P4MOVETYPE, P5MOVETYPE];

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

// ============== RUNTIME CONFIGURATION ==============

#[derive(Debug, Clone)]
pub struct Config {
    pub simulation_max: u32,
    pub x_max: usize,
    pub y_max: usize,
    pub player_number: usize,
    pub roll_max: u8,
    pub hole_probability: f64,
    pub player_movetypes: [u8; PLAYER_MAX],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            simulation_max: SIMULATION_MAX,
            x_max: X_MAX,
            y_max: Y_MAX,
            player_number: PLAYER_NUMBER,
            roll_max: ROLL_MAX,
            hole_probability: HOLE_PROBABILITY,
            player_movetypes: PLAYER_MOVETYPES,
        }
    }
}

impl Config {
    pub fn grid_size(&self) -> usize {
        self.x_max * self.y_max
    }

    pub fn grid_isize(&self) -> isize {
        self.grid_size() as isize
    }

    pub fn numbank_size(&self) -> usize {
        usize::div_ceil(self.grid_size(), self.player_number)
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Initialize config from command line arguments.
/// Call this at the start of main().
pub fn init_config_from_args() {
    let args: Vec<String> = std::env::args().collect();
    let mut config = Config::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--simulation-max" | "-s" => {
                i += 1;
                if i < args.len() {
                    config.simulation_max = args[i].parse().unwrap_or(SIMULATION_MAX);
                }
            }
            // "--x-max" | "-x" => {
            //     i += 1;
            //     if i < args.len() {
            //         let val: usize = args[i].parse().unwrap_or(X_MAX);
            //         if val > X_MAX {
            //             eprintln!(
            //                 "Warning: x-max {} exceeds compile-time X_MAX {}, clamping",
            //                 val, X_MAX
            //             );
            //         }
            //         config.x_max = val.min(X_MAX);
            //     }
            // }
            // "--y-max" | "-y" => {
            //     i += 1;
            //     if i < args.len() {
            //         let val: usize = args[i].parse().unwrap_or(Y_MAX);
            //         if val > Y_MAX {
            //             eprintln!(
            //                 "Warning: y-max {} exceeds compile-time Y_MAX {}, clamping",
            //                 val, Y_MAX
            //             );
            //         }
            //         config.y_max = val.min(Y_MAX);
            //     }
            // }
            "--roll-max" | "-r" => {
                i += 1;
                if i < args.len() {
                    config.roll_max = args[i].parse().unwrap_or(ROLL_MAX);
                }
            }
            "--hole-probability" | "--hole-prob" | "-hp" => {
                i += 1;
                if i < args.len() {
                    let val: f64 = args[i].parse().unwrap_or(HOLE_PROBABILITY);
                    config.hole_probability = val.clamp(0.0, 1.0);
                }
            }
            "-p1" => {
                i += 1;
                if i < args.len() {
                    config.player_movetypes[0] = args[i].parse().unwrap_or(P1MOVETYPE);
                }
            }
            "-p2" => {
                i += 1;
                if i < args.len() {
                    config.player_movetypes[1] = args[i].parse().unwrap_or(P2MOVETYPE);
                }
            }
            "-p3" => {
                i += 1;
                if i < args.len() {
                    config.player_movetypes[2] = args[i].parse().unwrap_or(P3MOVETYPE);
                }
            }
            "-p4" => {
                i += 1;
                if i < args.len() {
                    config.player_movetypes[3] = args[i].parse().unwrap_or(P4MOVETYPE);
                }
            }
            "-p5" => {
                i += 1;
                if i < args.len() {
                    config.player_movetypes[4] = args[i].parse().unwrap_or(P5MOVETYPE);
                }
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {}
        }
        i += 1;
    }

    let _ = CONFIG.set(config);
}

fn print_help() {
    println!("Usage: program [OPTIONS]");
    println!();
    println!("Options:");
    println!(
        "  -s,  --simulation-max <N>     Simulation iterations (default: {})",
        SIMULATION_MAX
    );
    // println!(
    //     "  -x,  --x-max <N>              Grid X dimension (default: {}, max: {})",
    //     X_MAX, X_MAX
    // );
    // println!(
    //     "  -y,  --y-max <N>              Grid Y dimension (default: {}, max: {})",
    //     Y_MAX, Y_MAX
    // );
    // println!(
    //     "  -n,  --player-number <N>      Number of players (default: {}, max: {})",
    //     PLAYER_NUMBER, PLAYER_MAX
    // );
    println!(
        "  -r,  --roll-max <N>           Maximum roll value (default: {})",
        ROLL_MAX
    );
    println!(
        "  -hp, --hole-probability <F>   Hole probability 0.0-1.0 (default: {})",
        HOLE_PROBABILITY
    );
    println!(
        "  -p1 <N>                       Player 1 movetype (default: {})",
        P1MOVETYPE
    );
    println!(
        "  -p2 <N>                       Player 2 movetype (default: {})",
        P2MOVETYPE
    );
    println!(
        "  -p3 <N>                       Player 3 movetype (default: {})",
        P3MOVETYPE
    );
    println!(
        "  -p4 <N>                       Player 4 movetype (default: {})",
        P4MOVETYPE
    );
    println!(
        "  -p5 <N>                       Player 5 movetype (default: {})",
        P5MOVETYPE
    );
    println!("  -h,  --help                   Print this help message");
    println!();
    println!("Avaliable movetypes:");
    println!("  0  Human         Terminal input.");
    println!("  1  Random        Random valid move.");
    println!("  2  Greedy        Highest-scoring move.");
    println!("  3  Monte Carlo   Monte Carlo flat search.");
}

/// Initialize with a custom config
pub fn init_config(config: Config) {
    let _ = CONFIG.set(config);
}

/// Get the current config (initializes to defaults if not set)
pub fn config() -> &'static Config {
    CONFIG.get_or_init(Config::default)
}

// ============== ACCESSOR FUNCTIONS (lowercase alternatives) ==============
pub fn simulation_max() -> u32 { config().simulation_max }
pub fn x_max() -> usize { config().x_max }
pub fn y_max() -> usize { config().y_max }
pub fn grid_size() -> usize { config().grid_size() }
pub fn grid_isize() -> isize { config().grid_isize() }
pub fn player_number() -> usize { config().player_number }
pub fn roll_max() -> u8 { config().roll_max }
pub fn hole_probability() -> f64 { config().hole_probability }
pub fn numbank_size() -> usize { config().numbank_size() }
pub fn player_movetypes() -> [u8; PLAYER_MAX] { config().player_movetypes }
pub fn p1movetype() -> u8 { config().player_movetypes[0] }
pub fn p2movetype() -> u8 { config().player_movetypes[1] }
pub fn p3movetype() -> u8 { config().player_movetypes[2] }
pub fn p4movetype() -> u8 { config().player_movetypes[3] }
pub fn p5movetype() -> u8 { config().player_movetypes[4] }

// ============== ORIGINAL HELPER FUNCTIONS (unchanged) ==============
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

pub const fn error_function(x: usize, y: usize) -> usize {
    (y * X_MAX) + x
}

pub fn get_valid_moves_from_takens(takens: &[bool; GRID_SIZE]) -> impl Iterator<Item = usize> {
    takens
        .iter()
        .enumerate()
        .filter(|(_, tile)| !*tile)
        .map(|(idx, _)| idx)
}
