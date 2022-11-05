use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guess game \n1. The system generates a secret number. \n2. You're to guess the secret number.");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop{
        println!("Enter the secret number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("Your guess {}, is too big",guess),
            Ordering::Less => println!("Your guess {}, is too small",guess),
            Ordering::Equal => { 
            println!("You Win,Congrats");
            break;
            }
        }
    }
}

