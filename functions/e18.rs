fn comp_mode(v: Vec<i32>) -> i32 {
    let mut sorted = v.clone();
    let mut curr: i32;
    let mut count: i32;
    let mut mode: i32;

    sorted.sort();

    curr = sorted[0];
    count = 0;
    mode = 0;
    for i in sorted {
        if curr == i {
            count += 1;
        } else {
            if count > mode {
                mode = count;
            }
            count = 0;
            curr = i;
        }
    }

    mode
}

fn main() {
    assert_eq!(comp_mode(vec![1, 2, 3, 3, 2, 2, 4, 4]), 2);
    println!("Success!");
}
