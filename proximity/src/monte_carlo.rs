use crate::constants::simulation_max;

impl Game {
    fn make_monte_carlo_flat_move(&mut self, player: Player) {
        self.start_time = Instant::now();
        let mut moves: &ArrayVec<usize, GRID_SIZE> = &self.grid.valid_moves;

        let mut moves_info: [MoveInfo; GRID_SIZE] = [MoveInfo {
            wins: 0,
            total: 100000000000000,
            uncertainty: 0, // uncertainty is 0.0-1.0 into 0-2^32 (4294967296)
        }; GRID_SIZE];

        let ptr = moves_info.as_mut_ptr() as usize;

        moves.par_iter().for_each(|tile| {
            let (current_wins, current_total) = self.evaluate(player, *tile, None);

            unsafe {
                // evil multithreading
                if *tile >= GRID_SIZE || self.grid.takens[*tile] {
                    panic!()
                }
                let moveinfo_ptr = (ptr as *mut MoveInfo).add(*tile);
                (*moveinfo_ptr).wins = current_wins as u64;
                (*moveinfo_ptr).total = current_total as u64;
            } // it's safe because each index is exclusive
        });
        self.moves = (simulation_max() as usize * (moves.len() * moves.len())) as usize;

        let best_move = moves_info
            .iter()
            .enumerate()
            .filter(|(a, _)| !self.grid.takens[*a])
            .max_by_key(|&item| (item.1.wins) * (TWO_POW_32) / (item.1.total))
            // ^^ janky ass solution to add more precision to integer division,
            // since float division isn't supported. (4294967296 is 2^32)
            .map(|(idx, _)| idx);

        self.add(player.roll(), player.id, best_move.unwrap());
        self.display(true);
    }

    // simulation stuff
    #[inline]
    fn run_single_rollout(&mut self, player: Player) -> u8 {
        self.simulation_loop(player)
    } // returns the winner from one randomly-played game

    fn evaluate(&self, player: Player, location: usize, simnum: Option<u32>) -> (u32, u32) {
        // chokepoint for parallelization
        // maybe just maybe pass down a shared clone instead of doing it every time
        // keep in mind safety
        let mut win_count: u32 = 0;
        let mut game_count: u32 = 0;
        let mut copy = self.clone();

        let sn = 
        if simnum == None {simulation_max()} 
        else {simnum.unwrap()};

        copy.add(player.roll(), player.id, location);

        loop {
            game_count += 1;
            if game_count >= sn {
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
}
