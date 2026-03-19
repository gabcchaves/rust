/* 1. Sintaxe e declaração de funções em Rust. */
//fn saudar() {
//    println!("Olá.");
//}

/* 2. Parâmetros e tipagem. */
//fn exibir_idade(idade: u32) {
//    println!("A idade informada é: {} anos.", idade);
//}

/* 3. Múltiplos parâmetros. */
fn imprimir_medidas(valor: f64, unidade: char) {
    println!("A medida é {}{}.", valor, unidade);
}

fn main() {
    imprimir_medidas(5.0, 'm');
}
