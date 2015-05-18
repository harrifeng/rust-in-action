use std::io;

fn main() {
    println!("Cuess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

////////////////////////////////////////////////////
// <===================OUTPUT===================> //
// $ cargo run                                    //
//      Running `target/debug/guessing_game`      //
// Cuess the number!                              //
// Please input your guess.                       //
// 6                                              //
// You guessed: 6                                 //
////////////////////////////////////////////////////
