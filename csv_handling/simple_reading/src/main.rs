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
    let rdr = Reader::from_path(path)?;
    Ok(())
}


fn main() {
    let result = open_file(String::from("foo.csv"));
    match result {
        Ok(_file) => {
            println!("OK.");
        },
        Err(error) => {
            println!("Error: {}", error);
        },
    };
}
