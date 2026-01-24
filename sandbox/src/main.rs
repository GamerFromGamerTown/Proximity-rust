use rand;
use std::io;

fn main() {
    println!("Welcome to the guessing game! The number is between 0 and 100. Guess the number!"); // TODO; make the range user-addable.
    let number: i32  = rand::random_range(1..101);
    let mut guesses: u32 = 0;

    loop {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let input_to_int: i32 = input.trim().parse().unwrap(); 

        if input_to_int > number {
            guesses += 1;
            println!("Wrong, too high! You're at {} guesses.", guesses)

        }
        else if input_to_int < number {
            guesses += 1;
            println!("Wrong, too low! You're at {} guesses.", guesses)  
        }
        else {
            guesses += 1;
            println!("Great guess! It only took {} guesses!", guesses);
            break
        }
    }
}
