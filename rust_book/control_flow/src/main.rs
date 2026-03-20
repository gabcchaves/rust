/* 1. Condicional básica. */
//fn test() {
//    let numero = 12;
//
//    if numero > 10 {
//        println!("O número é maior que 10.");
//    } else {
//        println!("O número é menor que 10.");
//    }
//}

/* 2. Múltiplas condições. */
//fn test(nota: u8) -> char {
//    if nota >= 90 {
//        'A'
//    } else if nota >= 80 {
//        'B'
//    } else if nota >= 70 {
//        'C'
//    } else {
//        'D'
//    }
//}

/* 3. if em uma intrução let */
// Considerando que if é uma expressão, então if retorna um valor e, portanto, pode ser usado em
// uma instrução let.
//fn test() -> String {
//    let condicao: bool = true;
//    let resultado = if condicao {
//        String::from("Sim")
//    } else {
//        String::from("Não")
//    };
//
//    resultado
//}

/* 4. Incompatibilidade de tipos no if. */
// O trecho de código a seguir não é possível de ser compilado por que envolve um retorno de dois
// possíveis tipos de dados a uma única variável. A variável não é tupla e, portanto, não pode
// receber diferentes tipos de dados.
fn test() {
    let condicao = true;
    let numero = if condicao { 5 } else { "seis" };
}

fn main() {
    test();
}
