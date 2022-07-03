fn main() {
    println!("escaped \\backslash\\ and \"quotes\"");
    println!(r"raw \backslash\, no quotes");
    println!(r#"raw \backslash\, raw "quotes", no hashquotes"#);
    println!(r##"raw \backslash\, raw "quotes", raw "#hashquotes#""##);
}
