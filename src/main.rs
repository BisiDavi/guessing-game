use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess_number = String::new();
    io::stdin().read_line( &mut guess_number).expect("Failed to read line");
    println!("You guessed {}", guess_number);
}

