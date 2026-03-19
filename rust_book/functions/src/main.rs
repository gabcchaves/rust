/* 1. Sintaxe e declaração de funções em Rust. */
//fn saudar() {
//    println!("Olá.");
//}

/* 2. Parâmetros e tipagem. */
fn exibir_idade(idade: u32) {
    println!("A idade informada é: {} anos.", idade);
}

fn main() {
    exibir_idade(24);
}
