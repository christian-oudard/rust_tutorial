use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::{self, Write};

const LIMIT: u32 = 100;

fn main() {
    let mut rng = thread_rng();
    let target: u32 = rng.gen_range(1..=LIMIT);

    println!("I'm thinking of a number between 1 and 100. Guess what it is!\n");

    loop {
        print!("Your guess: ");
        io::stdout().flush().expect("io");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read_line");

        let guess: u32 = match input.trim().parse() {
            Err(_) => {
                println!("That's not a valid number.\n");
                continue;
            }
            Ok(guess) => guess,
        };

        match guess.cmp(&target) {
            Ordering::Less => println!("Too low. Go higher.\n"),
            Ordering::Greater => println!("Too high. Go lower.\n"),
            Ordering::Equal => {
                println!("You guessed it! It was {}", target);
                break;
            }
        }
    }
}
