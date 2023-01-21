use std::ops::Add;

struct Pair {
    x: i32,
    y: i32,
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let pair1 = Pair { x: 10, y: 10 };
    let pair2 = Pair { x: 10, y: 10 };
    let pair3 = pair1.add(pair2);
}
