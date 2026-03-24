#![allow(unused)]
use csv::Reader;
use csv::Writer;
use std::error::Error;


/* Structs used */
// Reader //
// - from_path()
//
// Writer //
// - from_path()

fn open_csv() -> Result<(), Box<dyn std::error::Error>> {
//    let rdr = Reader::from_path("foo.csv");
    let mut wtr = Writer::from_path("foo.csv");
    let mut handle = match wtr {
        Ok(file) => {
            file
        },
        Err(error) => todo!(),
    };

    handle.write_record(&["a", "b"])?;
    handle.write_record(&["x", "y"])?;

    Ok(())

    //let _rdr_result = match rdr {
    //    Ok(file) => file,
    //    Err(error) => {
    //        //wtr.write_record(&["a", "b", "c"]);  
    //        //wtr.write_record(&["x", "y", "z"]);
    //        //wtr.flush();
    //    },
    //};
}

fn main() {
    open_csv();
}
