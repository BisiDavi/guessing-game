use rand::Rng;
use std::io;
use std::cmp::Ordering;

// guess game: 
// 1. a secret number is generated
// 2. you guess the secret number
// 3. your answer is compare with the secret number
// 4. when equals you win else you continue guessing

fn main(){
    println!("Welcome tp guessing game\nYou win when you guess the secret number correctly\nHint: Guess a number between 1 to 100");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop{
        println!("Enter number:");
        let mut guess = String::new(); 
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
       };

       match guess.cmp(& secret_number){
       Ordering::Greater => println!("Your guess is higher than the secret number"),
       Ordering::Equal => {
        println!("Congrats, you win");
        break;
        }
       Ordering::Less => println!("Your guess is less than the secret number")
       }
    }
}