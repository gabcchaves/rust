trait Shape {
    fn area(&self) -> f32;
}

struct Circle {
    radius: f32,
}

impl Circle {
    fn new(radius: f32) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius.powf(2.0) * std::f32::consts::PI
    }
}

fn main() {
    let circle = Circle::new(10.0);
    println!("{}", circle.area());
}
