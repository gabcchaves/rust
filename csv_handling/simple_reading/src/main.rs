use std::error::Error;
use csv::Reader;
/* 2. Leitura básica. */
// As estruturas principais do crate CSV são o Reader e Writer.
// Função para abrir arquivo e imprimir conteúdo na saída padrão.
//
// As implementações principais da estrutura Reader são:
// - from_path
// - from_reader

#[allow(unused_variables)]
fn open_file(path: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)
        .expect("Erro ao ler");
    
    // método records retorna iterador com os registros do arquivo.
    for result in rdr.records() {
        let record = result
            .expect("Erro ao ler");
        println!("{:?}", record);
    }

    Ok(())
}


fn main() {
    let result = open_file(String::from("foo.csv"))
        .expect("Erro ao ler");

    //// Tratamento de erro.
    //match result {
    //    Ok(_file) => {
    //        println!("OK.");
    //    },
    //    Err(error) => {
    //        println!("Error: {}", error);
    //    },
    //};
}
