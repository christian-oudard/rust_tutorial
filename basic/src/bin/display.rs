use std::fmt;

struct A;

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A")
    }
}

fn main() {
    let a = A;
    println!("{}", &a);
}
