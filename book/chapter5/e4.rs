#![allow(unused_variables)]
#![allow(dead_code)]

struct Counter {
    count: u8,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn increment(&mut self) -> Result<(), &'static str>{
        if self.count == 255 {
            Err("Erro: limite máximo atingido.")
        } else {
            self.count += 1;
            Ok(())
        }
    }

    fn decrement(&mut self) -> Result<(), &'static str> {
        if self.count > 0 {
            self.count -= 1;
            Ok(())
        } else {
            Err("Erro: Contagem igual a 0; Não pode ser menor.")
        }
    }

    fn display(&self) -> u8 {
        self.count
    }
}

fn main() -> Result<(), &'static str> {
    let mut counter = Counter::new();
    counter.increment()?;
    counter.increment()?;
    counter.increment()?;
    println!("{}", counter.display());
    counter.decrement()?;
    counter.decrement()?;
    counter.decrement()?;
    println!("{}", counter.display());

    counter.decrement()?;

    Ok(())
}
