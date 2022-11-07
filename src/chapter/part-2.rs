/*
    FUNCTIONS
    declared by the keyword 'fn'.

    rust is an expression based language.

    Statements are instructions that perform some actions and don't return a value;
    Expressions evaluate to a resulting value.


    LOOPS:
    1. loop 2. while 3. for
    
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

    let _mbd = 4;
    
    if true{
        println!("number was four")
    }


    loop_counter(5);
    loop_while_way();
    loop_for_way();
    loop_for_rev();
}

fn another_function(x:i32){
    println!("The value of x is: {}",x);           
}

fn plus_three(x:u32) -> u32{
    x +3
}

//loop
fn loop_counter(x:u32){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("Hello David, {}",counter);

        if counter == x {
            break counter * 2
        }
    };
    println! ("The result is: {}",result);
}

// while
fn loop_while_way(){
    let mut number = 3;

    while number != 0 {
        println!("LIFTUP-{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF");
}

fn loop_for_way(){
    let a = [10,20,30,40,50,60,70];

    for item in a.iter(){
        println!("the value is {}",item);
    }
}

fn loop_for_rev(){
    for element in (1..10).rev(){
        println!("{}!", element);
    }
    println!("Hello Didi");
}