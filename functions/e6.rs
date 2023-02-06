fn comp_avg(vec: Vec<f32>) -> f32 {
    let mut sum = 0.0;

    for i in &vec {
        sum += i;
    }

    (sum / vec.len() as f32 * 10.0).round() / 10.0
}

fn main() {
    assert_eq!(comp_avg(vec![1.2, 3.2, 0.0, 2.0, 3.0]), 1.9);
    println!("Success!");
}
