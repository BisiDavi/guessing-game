use std::io;

fn main(){
    fahrenheit_celsius(50.0);
//    let result = fib(9);
   generate_fib();
//    println!("result: {}",result);
}

//  Convert temperature between Fahrenheit and Celsius.
fn fahrenheit_celsius(fahrenheit:f64) {
    let degree = fahrenheit - 32.0;
    let quotient = 5.0/9.0;
    let celsius:f64 = degree * quotient;
    println!("{}F = {:?}C",fahrenheit, celsius);
}



fn fib(n:u32) -> u32{
    if n <= 0{
        return 0;
    }else if n == 1 {
        return 1;
    }
   fib(n-1) + fib(n-2)
}

fn generate_fib(){
    println!("To end the program, type 'exit' ");
    loop{
    let mut given_n = String::new();
    io::stdin().read_line(&mut given_n).expect("Failed to read line");
    if given_n.trim() == "exit"{
        break;
    }
    let given_n = match given_n.trim().parse(){
        Ok(num) => num,
        Err (_) => continue
    };
    let result = fib(given_n);
    println!("result: {}",result);
    }
}