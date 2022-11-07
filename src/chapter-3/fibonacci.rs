use std::io;
fn main(){
    println!("Welcome to fibonacci generator, enter a 'n', and it fibonacci will be generated, enter 'exit' to exit. ");
    loop{
        println!("Enter 'n':");
        let mut user_n = String::new();
        io::stdin().read_line(&mut user_n).expect("failed to read line");
        
        if user_n.trim() == "exit" {
            break;
        }
        
        let user_n = match user_n.trim().parse(){
            Ok (num) => num,
            Err (_) => continue
        };
        let fib_value = fib(user_n);
        println!("The fibonacci for the {} n series is {}:", user_n, fib_value);
    }
}

fn fib(n:u32) -> u32{
    if n <= 1 {
        return n;
    } else if n == 0 {
        return 1;
    }
    fib(n-1) + fib(n-2)
}