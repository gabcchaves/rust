use std::fmt;

struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {} ano(s).", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Gabriel"),
        age: 20
    };
    println!("{}", person);
}
