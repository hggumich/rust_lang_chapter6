enum IpAddr {
    V4(String),
    V6(String),
}

let home = IdAddr::V4(String::from("127.0.0.1"));

let loopback = IdAddr::V6(String::from("::1"));

fn main() {
    println!("Hello, world!");
}
