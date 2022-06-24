fn main() {
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
    
    println!("{}", _dec);
    println!("{}", _hex);
    println!("{}", _octal);
    println!("{}", _bin);
    println!("{}", _byte);
}
