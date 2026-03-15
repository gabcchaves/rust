#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn display(ip: IpAddr) {
    println!("{:?}", ip);
}

fn main() {
    let ip = IpAddr::V4(String::from("10.0.0.1"));
    display(ip);
}
