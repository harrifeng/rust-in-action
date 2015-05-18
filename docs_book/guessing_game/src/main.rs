extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////
// <===================OUTPUT===================>                                                        //
// $ cargo run                                                                                           //
//    Compiling guessing_game v0.1.0 (file:///Users/i309511/Code/rust-in-action/docs_book/guessing_game) //
//      Running `target/debug/guessing_game`                                                             //
// Guess the number!                                                                                     //
// The secret number is: 11                                                                              //
// Please input your guess.                                                                              //
// 7                                                                                                     //
// You guessed: 7                                                                                        //
///////////////////////////////////////////////////////////////////////////////////////////////////////////
