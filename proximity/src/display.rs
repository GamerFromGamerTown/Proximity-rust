impl Game {
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

            println!(
                "About {} moves per second, or {} ns/move. ({} sim/sec)",
                ((self.moves as f64 / elapsed).round()),
                ((elapsed * (10u64.pow(9)) as f64 / self.moves as f64).round()),
                ((((self.moves) as f64 / elapsed) / self.grid.valid_moves.len() as f64).round()),
            )
        }
    }
}
