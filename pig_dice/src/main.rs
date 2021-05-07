use rand::{thread_rng, Rng};

fn main() {

    let mut rng = thread_rng();

    let mut score: u32 = 0;
    loop {
        let roll: u32 = rng.gen_range(1..=6);
        if roll == 1 {
            score = 0;
            println!("1 (0)");
            break;
        }
        score += roll;
        println!("{} ({})", roll, score);

        if score >= 15 {
            break
        }
    }
    println!("score {}", score);

    println!("");
    for n in 1..=30 {
        let expected = expected_score_from_next_roll(n);
        println!("{} -> {:.1} ({:+.1})", n, expected, expected - n as f64);
    }
}

fn expected_score_from_next_roll(n: u32) -> f64 {
    (5. / 6.) * (n + (2 + 3 + 4 + 5 + 6) / 5) as f64
}
