/* 1. Sintaxe e declaração de funções em Rust. */
//fn saudar() {
//    println!("Olá.");
//}

/* 2. Parâmetros e tipagem. */
//fn exibir_idade(idade: u32) {
//    println!("A idade informada é: {} anos.", idade);
//}

/* 3. Múltiplos parâmetros. */
//fn imprimir_medidas(valor: f64, unidade: char) {
//    println!("A medida é {}{}.", valor, unidade);
//}

/* 4. Instrução vs expressão. */
/* Qual é a diferença fundamental entre uma instrução e uma expressão? */
// A diferença é que uma instrução não retorna valor algum, enquanto que uma expressão sempre
// retorna algum valor.
/* Por que não se pode atribuir uma intrução a outra instrução? [e.g.: let x = (let y = 10)]; */
// Justamente pelo fato de que uma instrução não retorna um valor. Como não retorna valor, não se
// pode atribuí-la uma outra instrução.

/* 5. O bloco como expressão. */
//fn expression_block() {
//    let y = {
//        let x = 3;
//        x + 1 // Esta linha retorna o valor de x + 1, 4.
//    };
//    println!("{}", y);
//}

/* 6. Valor de retorno básico. */
//fn cinco() -> i32 {
//    5
//}

/* 7. Operação de retorno. */
//fn somar_um(x: i32) -> i32 {
//    x + 1
//}

/* 8. O erro do ponto e vírgula. */
//fn somar_dois(x: i32) -> i32 {
//    x + 2 // O retorno implícito não deve incluir ponto e vírgula.
//}

/* 9. Snake Case */
//fn exampleFunction() {
//}
//
//fn example_function() {
//}

fn main() {
}
