fn to_string(v: Vec<i32>) -> String {
    // csn -> Comma separated numbers :)
    let mut csn = String::new();

    for n in v {
        csn.push_str(format!("{}", n).as_str());
        csn.push_str(", ");
    }
    csn.pop();
    csn.pop();

    csn
}

fn main() {
    assert_eq!(to_string(vec![1, 2, 3]), String::from("1, 2, 3"));
    println!("Success!");
}
