extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("please Input you guess!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

////////////////////////////////////////////////////
// <===================OUTPUT===================> //
// Guess the number!                              //
// The secret number is: 37                       //
// please Input you guess!                        //
// 342                                            //
// You guessed: 342                               //
////////////////////////////////////////////////////
