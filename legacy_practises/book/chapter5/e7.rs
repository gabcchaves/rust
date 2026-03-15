use std::cmp::PartialEq;

#[derive(Debug)]
struct Rectangle {
    width: u8,
    height: u8,
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        let area1 = self.width * self.height;
        let area2 = other.width * other.height;
        area1 == area2
    }
}

fn main() {
    let r1 = Rectangle { width: 10, height: 10 };
    let r2 = Rectangle { width: 10, height: 10 };
    assert_eq!(r1, r2);
}
