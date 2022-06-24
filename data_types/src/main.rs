use std::io::{self, Write};

fn main(){
    println!("Data types\n");
    /* Signed integers */
    // 8-bit signed integer
    let _x: i8 = 10;

    // 16-bit signed integer
    let _x: i16 = 10;

    // 32-bit signed integer
    let _x: i32 = 10;

    // 64-bit signed integer
    let _x: i64 = 10;

    // 128-bit signed integer;
    let _x: i128 = 10;

    // architecture's default size
    let _x: isize = 10;
    
    /* Unsigned integers */
    // 8-bit unsigned integer
    let _y: u8 = 10;

    // 16-bit unsigned integer
    let _y: u16 = 10;

    // 32-bit unsigned integer
    let _y: u32 = 10;

    // 64-bit unsigned integer
    let _y: u64 = 10;

    // 128-bit unsigned integer
    let _y: u128 = 10;

    // architecture's default size
    let _y: usize = 10;
    
    /* Integer literals */
    let _dec = 99_222;  // Decimal
    let _hex = 0xff;    // Hexadecimal
    let _octal = 0o77;  // Octal
    let _bin = 0b11;    // Binary
    let _byte = b'A';   // Byte
    
    println!("Integers");
    print!("{} {} {} {} {}\n\n", _dec, _hex, _octal, _bin, _byte);

    io::stdout().flush().unwrap();

    /* Floating point numbers */
    let _double = 2.0;      // f64
    let _float: f32 = 3.0;  // f32
    
    /* Numeric operations */
    // Addition
    let sum = 5 + 10;
    // Subtraction
    let diff = 95.5 - 4.3;
    // Multiplication
    let product = 4 * 30;
    // Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    // Remainder
    let remainder = 43 % 5;

    println!("Floating point numbers");
    print!("{} {} {} {} {} {}\n\n", sum, diff, product, quotient, floored, remainder);

    /* Boolean */
    let t = true;
    let f: bool = false;

    println!("Booleans");
    print!("{} {}\n\n", t, f);

    /* Character */
    let c = 'z';
    println!("Character");
    print!("{}\n\n", c);
}
