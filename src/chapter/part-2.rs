/*
    FUNCTIONS
    declared by the keyword 'fn'.

    rust is an expression based language.

    Statements are instructions that perform some actions and don't return a value;
    Expressions evaluate to a resulting value.

    
*/

fn main(){
    println!("Hello 9ja");

    another_function(20);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    fn ten() -> u32{
       10 
    }

    let d = ten();

    let g = plus_three(20);

    println!("The value of d is: {}",d);
    println!("The value of g is: {}",g);
}

fn another_function(x:i32){
    println!("The value of x is: {}",x);           
}

fn plus_three(x:u32) -> u32{
    x +3
}