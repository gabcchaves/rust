use std::io;

fn main() -> Result<(), io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    println!("{}", &input);

    Ok(())
}
