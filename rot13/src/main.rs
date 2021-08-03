fn main() {
    println!("{}", rot13("YRIRY GJB CNFFJBEQ EBGGRA".into()));
}

fn rot13(s: String) -> String {
    s.chars().map(rot13_char).collect()
}

const LO: u8 = 'A' as u8;
const HI: u8 = 'Z' as u8 + 1;

fn rot13_char(c: char) -> char {
    let mut x = c as u8;
    if LO <= x && x < HI {
        x -=  LO;
        x = (x + 13) % 26;
        x += LO;
    }
    x as char
}

