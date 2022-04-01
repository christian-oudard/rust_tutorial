fn main() {
    let y = f(1 + 2);
    dbg!(y);
}

fn f(x: i32) -> i32 {
    x + 1
}
