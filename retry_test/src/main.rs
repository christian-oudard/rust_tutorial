use retry::{retry, delay::Fibonacci};
use std::io::{self, Write};

fn main() {
    let delays: Vec<_> = Fibonacci::from_millis(10).take(20).collect();
    dbg!(&delays);
    let result = retry(delays, || {
        print!(".");
        io::stdout().flush().unwrap();
        Err::<(), &str>("no")
    });

    println!();
    assert!(result.is_err());
}
