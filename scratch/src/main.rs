fn main() {
    get_closure()();
}

fn get_closure() -> impl Fn() {
    || {
        println!("hello from the closure side");
    }
}

