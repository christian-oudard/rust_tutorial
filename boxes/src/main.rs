use std::error::Error;
use std::fs;

fn main() {
    print!("{}", read_file().unwrap());
}

fn read_file() -> Result<String, Box<dyn Error>> {
    let bytes = fs::read("missing")?;
    let s = String::from_utf8(bytes)?;
    Ok(s)
}
