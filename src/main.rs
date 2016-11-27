extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number (1 - 101):)");

    println!("Start guessing:");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("guess.trim() is {}", guess);

        let guess: u32 = guess.trim().parse()
            .expect("It has to be a number!!!");

        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small! Guess again"),
            Ordering::Greater => println!("Too big! Guess again"),
            Ordering::Equal => {
                println!("You win!s");
                break;
            }
        }
    }
}
