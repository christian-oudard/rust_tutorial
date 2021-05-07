fn main() {
    println!("{:?}", maybe_parse_int(None));
    println!("{:?}", maybe_parse_int(Some("123".into())));
    println!("{:?}", maybe_parse_int(Some("invalid".into())));
}

fn maybe_parse_int(maybe_s: Option<String>) -> Option<i64> {
    match maybe_s {
        Some(s) => s.parse().ok(),
        None => None,
    }
}
