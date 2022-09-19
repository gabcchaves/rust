use std::io;
use std::str::FromStr;

fn main() {
    let scn = io::stdin();
    let (mut a, mut b, mut c) = (
        String::new(),
        String::new(),
        String::new()
    );

    loop {
        match scn.read_line(&mut a) {
            Ok(..) => {}
            Err(error) => {
                println!("Error: {error}");
            }
        }
        if f64::from_str(&a).is_ok() {
            break;
        }
    }

    match scn.read_line(&mut b) {
        Ok(..) => {}
        Err(error) => {
            println!("Error: {error}");
        }
    }
    match scn.read_line(&mut c) {
        Ok(..) => {}
        Err(error) => {
            println!("Error: {error}");
        }
    }
}
