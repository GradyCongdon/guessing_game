
// 3.2 Data Types
// every value has a data type that must be known at compile time (staticlly typed)
// Either scalar or compound
// Compiler usually can infer but if there are multiple possible types, we must annotate
// let guess: u32 = "42".parse().expect("NaN") 
// parse returns Result + multiple number types
// annotate guess with u32 to avoid no_type_annotation error

// Scalar types
// single value - 4 primary types
// int, float, boolean, char
// integers use i8-128 or isize for signed
// u8-128 or usize
// size suffix uses cpu arch (32 bit / 64 bit)

// signed stored using two's complement
// most significant digit is used to signal negative (1) or pos (0)
// to convert to 2's:
// invert/flip all bits
// add a place value of 1 to the inverted number
// take 6 - 0110
// invert - 1001
// add 1  - 1010
// 1 (-8) + 0 (4) + 1 (2) + 0 (1) = 6

// number literals can also specify type
// 98_222 - decimal with seperator _
// 0xff - hex
// 0o777 - octal
// 0b1111_0000 - binary
// b'A' - byte u8 only

// i32, u32 good defaults
// isize, usize useful when indexing a colletion

// debug mode will check for integer overflow
// i.e. adding 1 to u8 at 255
// will panic (exit with error)
// release mode will not check, but wrap with 2's complement
// u8 at 256 -> 0, 257 -> 1
// wrapping should be explicitly handled
// wrapping_add: wraps w/o panic
// checked_add: Returns Result Some/None
// overflowing_add: returns val and boolean wrap flag
// saturating_add: clamps at min or max (stay at 255)


// Floating point numbers
// f32, f64 - 64 the default and both are signed
// IEEE-754, 32 single precision, 64 double 
// integer divisoin rounds to the nearest int

// Boolean
// true or false
// typed with x: bool

// Character
// single quoted
// four bytes
// ASCII, emoji, zero-width, chinese, etc all valid


// Compound types

// Tuple
// fixed length, cannot be change size after declaration
// let tup: (i32, f64, u8) = (500, 6.4, 1);
// let (x, y, z) = tup; // destructure to vars
// tup.0 tup.1 tup.2 for direct access via index
// () unit - special tuple wiht no values
// expr return unit value by default if nothing else is returned

// Array
// fixed length collection of multiple values all with the same type
// let a: [i32; 3] = [1, 2, 3];
// Stack allocated
// let a = [3; 5]; // initalization shorthand
// indexing outside can be caught by compiler somtimes
// but will produce a runtime panic if not caught 
// other low-level langs do not protect and can access invalid mem
// rust will immediately exit instead of allowing 

fn main() {
    let i = 5;
    let a = [3;3];
    let x = a[i];
    println!("{x}")
}













// 3.1
fn threeOne() {
    let mut x = 5;
    x = 6;
    println!("{}", x);
    shadow();
}

// const must be annotated
// can be declared in any scope including global, valid only in scope declared
// can only be set to a constant expr, no runtime computations
// const THREE_HOURS: u32 = 60 * 60 * 3; // all good, still constant


// shadowing - declaring a var with the same name as a previous one
fn shadow() { 
    let x = 5;
    // shadow by using let, x = x + 1 would cause compile error
    let x = x + 1;
    
    // new var created, type can be changed, mut will not work here
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    {
        let x = x * 2;
        println!("inner scope: {x}"); // 12
    } // inner shadow ends
    println!("outer scope: {x}"); // 6
}