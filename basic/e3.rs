use std::io;

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    let f: f32;
    let c: f32;

    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    c = input.parse().unwrap();
    f = c * 1.8 + 32.0;

    println!("{}", f);

    Ok(())
}
