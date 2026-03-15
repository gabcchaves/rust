fn main() {
    println!("{}", special_triple());
}

fn special_triple() -> i64 {
    let mut b: i64 = 4;
    loop {
        for a in 1..b {
            let c = f32::sqrt( ((a as i64).pow(2) + b.pow(2)) as f32 );
            if (a as f32) + (b as f32) + c == 1000.0 {
                return a*b*(c as i64);
            }
        }
        b += 1;
    }
}
