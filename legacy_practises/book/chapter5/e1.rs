struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(self) -> f32 {
        self.width * self.height
    }
}

fn main() {
    let square = Rectangle { width: 10.0, height: 10.0 };
    println!("{}", square.area());
}
