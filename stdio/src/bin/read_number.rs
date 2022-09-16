use std::io;
use std::str::FromStr;

fn main() {
    let scn = io::stdin();
    let mut str_n = String::new();

    println!("Please enter a number.");
    scn.read_line(&mut str_n);

    let n = i32::from_str(str_n.as_str().trim()).unwrap();
}
