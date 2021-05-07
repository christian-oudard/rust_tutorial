fn main() {
    println!("{:?}", default_empty(Some("hey".to_string())));
    println!("{:?}", default_empty(None));
}

fn default_empty(s: Option<String>) -> String {
    // match s {
    //     Some(s) => s,
    //     None => "".to_string(),
    // }
    s.unwrap_or("".to_string())
}
