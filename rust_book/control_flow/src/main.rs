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
//fn test() {
//    let condicao = true;
//    let numero = if condicao { 5 } else { "seis" };
//}

/* 5. loop infinito com loop. */
//fn test() {
//    let mut contador = 0;
//    loop {
//        contador = contador + 1;
//        println!("processando...");
//        if contador >= 5 {
//            break;
//        }
//    }
//}

/* 6. Retornando valores de loops. */
//fn test() -> i32 {
//    let mut contador = 0;
//    loop {
//        contador = contador + 1;
//        if contador >= 10 {
//            break contador * 2
//        }
//    }
//}

/* 7. Rótulos de laço. */
//#[allow(unreachable_code)]
//fn test() -> String {
//    'externo: loop {
//        loop {
//            break 'externo;
//        };
//        return String::from("Falha");
//    };
//    String::from("Êxito")
//}

/* 8. Estrutura condicional com while. */
//fn test() {
//    let mut contador: u8 = 3;
//    while contador >= 1 {
//        println!("{}", contador);
//        contador = contador - 1;
//    };
//    println!("Decolar!");
//}

/* 9. Percorrendo coleções com for. */
//fn test() {
//    let elementos = [10, 20, 30, 40, 50];
//    for number in elementos {
//        println!("{} ", number);
//    }
//    // For loop é considerado mais rápido porque não há necessidade de comparar o índice com o
//    // comprimento do arranjo. Além de ser considerado mais rápido, o for loop é considerado mais
//    // seguro, pois elimina a possibilidade de ultrapassar os limites inferior e superior do
//    // arranjo.
//}

/* 10. O uso de rev() e ranges. */
//fn test() {
//    for number in (1..4).rev() {
//        println!("{} ", number);
//    }
//}

/* 11. Controle de fluxo e divisibilidade. */
fn test() {
    for numero in 1..20 {
        if numero % 3 == 0 && numero % 5 == 0 {
            println!("FizzBuzz");
        } else if numero % 3 == 0 {
            println!("Fizz");
        } else if numero % 5 == 0 {
            println!("Buzz");
        }
    }
}

fn main() {
    test();
}
