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
i. integer ii. floating point iii. character iv. boolean

INTEGER
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

*/

fn main(){
    let x = 10;
    let x = x+ 2;
    let x = x * 3;

    println!("X: {}", x);
}