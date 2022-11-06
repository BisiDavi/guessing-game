/* 
CONSTANTS & SHADOWING
variables are immutable by default
they can be made mutable by 'mut'
mutable type
constant are forever immutable and you can't use 'mut' on them.

const APP_SUBSCRIBERS:u32 = 5_000   (this a constant)
let points: f64 = 4.0 (this a variable)

the type of a constant must be annotated

constants can be declared in any scope, including global scope.

constants are only set to constant expression

DATA TYPES 
1. Scalar (of one type)
2. Compound

1. SCALAR 
a. integer b. floating point c. boolean d. character

A. INTEGER
i. Signed Integers (i, stores both negative and positive integers from -(2^(n-1)) to (2^(n-1) - 1).
ii. Unsigned Integers (u, stores positive integers only from 0 to 2^n - 1).

Number Literals
i. Decimal ii. Hex iii. Octal iv. Binary v. Byte

INTEGER OVERFLOW
when variable value passes the type bound an overflow occurs.
let y:u8 = 257;
y is an overflow;
if in release mode (--release), rust doesn't check for integer overflow that causes panics.
If an overflow occurs at release, Rust performs two's complement wrapping.
256 becomes 0 , 257 becomes 1, hence giving you different value.

B. FLOATING POINT TYPES
Floating Points are decimal points. They are represented as f32 and f64.
f32 is single precision float.
f64 is double precision float, f64 is the default type for Floating point in Rust.

C. BOOLEAN
Boolean - true , false
Boolean are one byte in size

D. CHARACTER
char type is rust alphabetic type, represented with ' ' (single quote) unlike string ("" , double quotes).
char type is four bytes 


2. COMPOUND TYPES
A. TUPLE TYPE
B. ARRAY TYPE

A. TUPLE
A way of grouping multiple variables together with different types, tuples have fixed length.
let tup_1: (u32,char, bool) = (500, 'b', false);

B. ARRAY
Arrays are fixed same data type group.
let ab:[i32;4] = [1,2,4,5]
Arrays cannot reduce/shrink or grow/increase.
vectors can grow/reduce in size.

Array is a single chunk of memory allocated on the stack.

*/

fn main(){
    let x = 10; //u32 (default)
    let x = x+ 2;
    let x = x * 3;
    let a = 1.0; //f64 (default)
    let y= a + 4.5;
    let fx = 'f'; //char
    let smile = '🤣';
    let tup_1: (u32,char, bool) = (500, 'b', false);
    let (r,s,t) = tup_1;
    let phones_os = ["andriod", "ios"];
    let same = [3;3];

    println!("X: {}, Y: {}", x, y);
    println!("fx: {}, smile: {}", fx, smile);
    println!("tuple: {:?}, index(2):{:?}", tup_1, tup_1.2);
    println!("r:{:?}, s:{:?}, t:{:?}", r,s,t);
    println!("Phone OS: {:?}, same: {:?}", phones_os, same);
}