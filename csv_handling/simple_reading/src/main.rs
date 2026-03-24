use std::error::Error;
use csv::Reader;
use csv::ReaderBuilder;
/* 2. Leitura básica. */
// As estruturas principais do crate CSV são o Reader e Writer.
// Função para abrir arquivo e imprimir conteúdo na saída padrão.
//
// As implementações principais da estrutura Reader são:
// - from_path
// - from_reader

#[allow(dead_code)]
#[allow(unused_variables)]
fn open_file(path: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    
    // método records retorna iterador com os registros do arquivo.
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

#[allow(unused_variables)]
fn read_semicolon(path: String) -> Result<(), Box<dyn Error>> {
    // Leitura de arquivo CSV cujo separador é ';'.
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}


fn main() {
    let result = read_semicolon(String::from("foo2.csv"));

    // Tratamento de erro.
    match result {
        Ok(_file) => {
            println!("OK.");
        },
        Err(error) => {
            println!("Error: {}", error);
        },
    };
}
