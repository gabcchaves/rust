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
fn test() -> String {
    let condicao: bool = true;
    let resultado = if condicao {
        String::from("Sim")
    } else {
        String::from("Não")
    };

    resultado
}

fn main() {
    println!("{}", test());
}
