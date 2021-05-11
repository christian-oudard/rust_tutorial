use std::env;

fn main() {
    let password = env::var("PASSWORD").unwrap_or("".to_string());
    println!("pw: {}", password);
}
