fn main() {
    let now = std::time::Instant::now();
    let mut i = 0;
    loop {
        i += 1;
        if i >= 100_000_000 {
            break;
        }
    }
    let end = now.elapsed();
    println!("{}", end.as_secs_f32());
}
