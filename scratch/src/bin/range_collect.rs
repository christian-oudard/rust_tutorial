fn main() {
    println!(
        "{}",
        (1..=10).rev().map(|n| n.to_string()).collect::<Vec<_>>().join(", ")
    );
    dbg!((1..=10).collect::<Vec<_>>().windows(3).collect::<Vec<_>>());

    dbg!((-5..=5).collect::<Vec<_>>());
}
