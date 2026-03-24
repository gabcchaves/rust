#![allow(unused)]
use csv::Reader;
use std::error::Error;

fn open_csv() {
    let rdr = Reader::from_path("foo.csv");
    let _rdr_result = match rdr {
        Ok(file) => file,
        Err(error) => panic!("Arquivo inexistente:\n\t{}", error),
    };
}

fn main() {
    open_csv();
}
