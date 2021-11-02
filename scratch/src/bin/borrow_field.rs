#[derive(Debug)]
struct S {
    field: String,
}

impl S {
    fn f(&mut self) -> &str {
        self.field = "B".to_string();
        &self.field
    }
}

fn main() {
    let mut x = S {
        field: "A".to_string(),
    };
    let a = &x.field;
    let a = a.clone();  // Must clone, or it holds a borrow.
    let b = x.f();
    println!("{} {}", a, b);
}
