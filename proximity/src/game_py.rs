#[pymethods]
impl Game {
    // core
    fn add(&mut self, value: u8, owner: u8, location: usize) {
        /* removed bounds checks
        if self.grid.takens[location] {
            panic!("AAAHHHH");
        } */

        self.grid.values[location] = value;
        self.grid.owners[location] = owner;
        self.grid.takens[location] = true;

        self.remove_valid_tile(location);

        self.grid.update_neighbors(value, owner, location);
        // another third is in update_neighbors

        self.players[(owner - 1) as usize].turn += 1;
        self.players[(owner - 1) as usize].score += value as usize;
        self.grid.turn += 1;
    }

    fn remove_valid_tile(&mut self, location: usize) {
        let mut lo = 0;
        let mut hi = self.grid.valid_moves.len() - 1;
        let mut itr: usize = 0;

        loop {
            // consider making the search start at location / size
            let mut mid = lo + (((hi - lo)) / 2);
            let v = self.grid.valid_moves[mid];
            
            if v == location {
                self.grid.valid_moves.remove(mid);
                break
            }
            else if v < location {
                lo = mid + 1
            }
            else {
                hi = mid - 1
            }

            itr += 1;
            if itr > GRID_SIZE {panic!("binary search ran for far too long, probably stuck")}
        }
    }

    fn get_raw_values(&self) -> [u8; GRID_SIZE] {
        self.grid.values.clone()
    }
    fn get_raw_owners(&self) -> [u8; GRID_SIZE] {
        self.grid.owners.clone()
    }
    fn get_raw_takens(&self) -> [bool; GRID_SIZE] {
        self.grid.takens.clone()
    }

    fn roll_py(&self, owner: u8) -> u8 {
        self.players[(owner as usize) - 1].numbank[self.grid.turn]
    }

    fn is_terminal_py(&self) -> bool {
        self.grid.is_terminal()
    }
}
