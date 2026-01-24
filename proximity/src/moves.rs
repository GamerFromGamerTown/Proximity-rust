impl Game {
    // move types
    fn make_random_move(&mut self, player: Player, rng: &mut impl Rng) {
        // CRITICAL: this line causes most cumtime (probably)
        let mut moves: &ArrayVec<usize, GRID_SIZE> = &self.grid.valid_moves;

        let chosen_move: usize = *moves.choose(rng).expect("Game is not terminal.");

        self.add(player.roll(), player.id, chosen_move);
    }

    fn make_greedy_move(&mut self, player: Player) {
        let mut moves: &ArrayVec<usize, GRID_SIZE> = &self.grid.valid_moves;

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

    fn make_human_move(&mut self, player: Player) {
        self.display(false);
        loop {
            println!(
                "Please choose an X and Y to place the tile. Add a comma afterwards, too. Example: 5,3, \n Your roll is {}",
                player.roll()
            );

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let separated_input: Vec<&str> = input.split(",").collect();

            let xy: Vec<usize> = separated_input
                .iter()
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            println!("{:?}", xy);
            if xy.len() != 2 {
                println!("Invalid input. Please try again.");
                continue;
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
            break;
        }
    }

    // misc
    fn make_move(&mut self, player: Player) {
        let mut rng = rng();
        if player.move_type == 0 {
            self.make_human_move(player);
        } else if player.move_type == 1 {
            self.make_random_move(player, &mut rng);
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
