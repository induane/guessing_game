extern crate rand;

// use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    game();

    loop {
        println!("\nDo you want to play again? [y/n]");
        let mut response = String::new();
        io::stdin().read_line(&mut response)
            .ok()
            .expect("Uhh what did you say??");

        if response.trim() == "N" {
            break;
        }
        if response.trim() == "n" {
            break;
        }
        if response.trim() == "no" {
            break;
        }
        if response.trim() == "No" {
            break;
        }
        game();
    }

}

fn game() {
    clear_screen();
    println!("########################################");
    println!("# The 'you wish you were psychic' Game #");
    println!("########################################\n\n");

    println!("Enter the upper bounds for the game:");
    let mut upper_bound = String::new();
    io::stdin().read_line(&mut upper_bound)
        .ok()
        .expect("Can't figure out what you said...");

    let upper_bound: u32 = upper_bound.trim().parse()
        .ok()
        .expect("Invalid upper bounds value - try a real number..");

    let picked_number = rand::thread_rng().gen_range(1, upper_bound + 1);
    let mut guess_tally: u32 = 0;

    println!("A game where you guess what number I picked!...");

    loop {
        println!("Please input your guess [1-{0}]: ", upper_bound);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Unable to read input");

        guess_tally += 1;

        if guess.trim() == "exit" {
            println!("I guess you're giving up. Haha loser!");
            break;
        }

        if guess.trim() == "quit" {
            println!("I guess you're giving up. Haha loser!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&picked_number) {
            Ordering::Less    => {
                println!("Too small");
            }
            Ordering::Greater => {
                println!("Too big");
            }
            Ordering::Equal   => {
                println!("You picked the right number in {0} guesses!", guess_tally);
                break;
            }
        }
    }
}


// Clear the terminal window
fn clear_screen() {
    io::stdout().write_all(b"\x1b[2J\x1b[1;1H").unwrap();
}
