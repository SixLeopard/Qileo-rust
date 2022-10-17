#![allow(nonstandard_style)]

use std::io;
use rand::Rng;

fn main() {
    loop{
        let mut playAgain = String::new();

        io::stdin().read_line(&mut playAgain).expect("Failed to read input");

        let playAgain: bool = playAgain.trim().parse().expect("not bool");

        if playAgain{
            GameLoop();
        }else{
            break;
        }
    }
    
}

fn GameLoop(){
    let number = rand::thread_rng().gen_range(1..100);
    loop {
        println!("Guess a Number");

        println!("Please Input your Guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        if number == guess {
            println!("Correct!");
            break;
        }

        if number < guess{
            println!("To High!");
        }

        if number > guess{
            println!("To Low!");
        }
    }
}