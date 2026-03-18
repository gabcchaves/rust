fn main() {
    //// Testing interger overflow.
    //let x: u8 = 256;
    //println!("Value of x is {x}");
    //// It throws an error, indicating that u8 variables shall be in the range of 0..=255.

    //// Which data type would Rust compiler infer?
    //let x = 3.1415;
    //println!("Compiler infered that x is of type {}.", std::any::type_name_of_val(&x));

    // Now, let's try operating with variables of different types.
    //let x: u8 = 10;
    //let w: f32 = 2.5;
    //let r: f32 = x as f32 + w;
    //println!("{r}");

    // 2. Qual tipo inteiro o compilador do Rust infere quando se inicializa uma variável inteira
    //    sem notação de tipo?
    let x = 42;
    println!("{}", std::any::type_name_of_val(&x));
    // O programa exibe o tipo i32. Portanto, esse é o tipo inteiro inferido pelo compilador do
    // Rust. Por quê?
    
    // O seguinte trecho causa um erro de compilação, o que indica que o compilador do Rust sempre
    // escolhe o tipo i32, mesmo que o número atrelado à variável não caiba em tal tipo.
    //let y = 999999999999999;
    //println!("{}", std::any::type_name_of_val(&y));


    // A solução, nesse caso, é definir o tipo de inteiro como i64.
    let z: i64 = 999999999999999;
    println!("{}", std::any::type_name_of_val(&z));
}
