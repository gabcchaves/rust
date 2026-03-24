use std::error::Error;
use csv::Reader;
use csv::ReaderBuilder;
use csv::StringRecord;
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


#[allow(dead_code)]
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


#[allow(dead_code)]
#[allow(unused_variables)]
fn read_ignore_headers(path: String) -> Result<(), Box<dyn Error>> {
    // Ignorar cabeçalhos na leitura.
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}


#[allow(dead_code)]
// Ler uma única linha.
fn read_line(path: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    
    let mut record = StringRecord::new();

    //// read_record retorna valor booleano.
    //if rdr.read_record(&mut record)? {
    //    println!("{:?}", record);
    //    Ok(())
    //} else {
    //    Err((error))
    //}
    
    // ERRADO:
    //match rdr.read_record(&mut record) {
    //    Ok(record) => {
    //        println!("{:?}", record);
    //    },
    //    Err(error) => return Err(error),
    //}

    rdr.read_record(&mut record)?;
    println!("{:?}", record);

    Ok(())
}


// Contar número de colunas do arquivo.
fn count_cols(path: String) -> Result<(), Box<dyn Error>> {
    // Deve consultar apenas o cabeçalho.
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    let mut header_line = StringRecord::new();

    rdr.read_record(&mut header_line)?;
    println!("{:?} colunas.", header_line.len());

    Ok(())
}


fn main() {
    let result = count_cols(String::from("foo.csv"));

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
