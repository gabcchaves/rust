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

    /*
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
    */


    /*
     * 3. Precisão do ponto flutuante.
    // O compilador do Rust sempre infere o tipo f64, por algum motivo...
    let x = 2.0;
    println!("{}", std::any::type_name_of_val(&x));

    // Para usar uma precisão menor, deve-se anotar o tipo específico.
    let y: f32 = 2.0;
    println!("{}", std::any::type_name_of_val(&y));
    */


    /* 4. Operações com tipos de dados diferentes.
    // Por que que o seguinte trecho não é compilado exitosamente?
    // Por que que o compilador não consegue inferir o tipo de z?
    let x: f32 = 2.0;
    let y: i32 = 2;
    let z = x + y as f32;
    println!("{}", std::any::type_name_of_val(&z));
    // Como emular a tipagem dinâmica no Rust?
    // Pode-se usar casting na variável cujo conjuto numérico é subconjunto do outro.
    */

    /* 5. Divisão de números inteiros.
     * Qual é o comportamento do compilador do Rust ao tentar dividir números inteiros de modo a
     * resultar em número de ponto flutuante?
    let x = 5;
    let y = 2;
    let z = x / y;
    println!("{}", z);
    // O compilador realiza o truncamento do resultado, para preservar o tipo inteiro dos dois
    // operandos.
    println!("{}", std::any::type_name_of_val(&z));
    */

    /* 6. Tipo booleano.
     *Teste das operações booleanas com variáveis desse tipo.
     *
    let possui_chave: bool = true;
    let porta_aberta: bool = false;
    let pode_entrar: bool = possui_chave || porta_aberta;

    println!("Possui chave? {}", {
        if possui_chave { "Sim." } else { "Não." }
    });

    println!("A porta está aberta? {}", {
        if porta_aberta { "Sim." } else { "Não." }
    });

    println!("Pode entrar? {}", {
        if pode_entrar { "Sim." } else { "Não." }
    });
     */
    
    /* 7. Tipo caractere Unicode.
     * Quantos bytes ocupa na memória?
     * Suporta ASCII somente?
     */
    let c: char = '😀';
    println!("Tamanho do caractere: {} bytes.", std::mem::size_of::<char>());
    println!("Emoji: {}.", c);
    // Em suma, sim, tipo char ocupa 4 bytes em memória e suporta emojis.
}
