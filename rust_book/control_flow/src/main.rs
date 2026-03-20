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
fn test(nota: u8) -> char {
    if nota >= 90 {
        'A'
    } else if nota >= 80 {
        'B'
    } else if nota >= 70 {
        'C'
    } else {
        'D'
    }
}

fn main() {
    println!("{}", test(100));
}
