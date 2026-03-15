fn main() {
    let mut largest: i32 = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let str_prod: String = (i * j).to_string();
            let str_prod_rev: String = str_prod.chars().rev().collect::<String>();
            if str_prod == str_prod_rev && i * j > largest {
                largest = i * j;
            }
        }
    }
    println!("{}", largest);
}
