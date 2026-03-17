fn main() {
    //// Testing interger overflow.
    //let x: u8 = 256;
    //println!("Value of x is {x}");
    //// It throws an error, indicating that u8 variables shall be in the range of 0..=255.

    //// Which data type would Rust compiler infer?
    //let x = 3.1415;
    //println!("Compiler infered that x is of type {}.", std::any::type_name_of_val(&x));

    // Now, let's try operating with variables of different types.
    let x: u8 = 10;
    let w: f32 = 2.5;
    let r: f32 = x as f32 + w;
    println!("{r}");
}
