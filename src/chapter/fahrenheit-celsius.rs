use std::io;

fn main(){
    println!("welcome, convert fahrenheit to celsius by, enter exit to quit");

    loop{
        println!("Enter fahrenheit");
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit).expect("failed to read line");
        if fahrenheit.trim() == "exit"{
            break;
        }
        let fahrenheit  = match fahrenheit.trim().parse(){
            Ok (num) => num,
            Err (_) => continue
        };
        println!("{}F, fahrenheit",fahrenheit);
        fahrenheit_celsius(fahrenheit);
    }
}

fn fahrenheit_celsius(fahrenheit:f64){
    let celsius = (fahrenheit - 32.0) * (5.0/9.0);
    println!("{}F = {}C", fahrenheit, celsius);
}