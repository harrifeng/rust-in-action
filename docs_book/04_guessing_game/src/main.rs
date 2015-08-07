use std::io;

fn main() {
    println!("Guess the number!");

    println!("please Input you guess!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}

