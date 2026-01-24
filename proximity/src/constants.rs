use arrayvec::ArrayVec;

pub const RECORD_WINLOSS: bool = true;
pub const SIMULATION_MAX: u32 = 5000; // replace this with an error function! see error_function_idea
pub const TWO_POW_32: u64 = 4294967296;
pub const X_MAX: usize = 10;
pub const Y_MAX: usize = 8;
pub const GRID_SIZE: usize = X_MAX * Y_MAX;
pub const GRID_ISIZE: isize = GRID_SIZE as isize;

pub const PLAYER_NUMBER: usize = 2;
pub const PLAYER_MAX: usize = 5;

pub const NUMBANK_SIZE: usize = usize::div_ceil(GRID_SIZE, PLAYER_NUMBER); // ceiling division!
pub const ROLL_MAX: u8 = 20;
pub const HOLE_PROBABILITY: f64 = 0.0; // 0.0 >= n >= 1

pub const P1MOVETYPE: u8 = 3;
pub const P2MOVETYPE: u8 = 3;
pub const P3MOVETYPE: u8 = 1;
pub const P4MOVETYPE: u8 = 1;
pub const P5MOVETYPE: u8 = 1;

pub const PLAYER_MOVETYPES: [u8; PLAYER_MAX] =
    [P1MOVETYPE, P2MOVETYPE, P3MOVETYPE, P4MOVETYPE, P5MOVETYPE];

// 0 is human
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

pub const fn error_function(x: usize, y: usize) -> usize {
    (y * X_MAX) + x
}

pub fn get_valid_moves_from_takens(takens: &[bool; GRID_SIZE]) -> impl Iterator<Item = usize> {
    takens
        .iter()
        .enumerate()
        .filter(|(_, tile)| !*tile) // only include true tiles
        .map(|(idx, _)| idx) // ignore the tile bool itself
}
