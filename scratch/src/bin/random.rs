use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    let x: f64 = rng.gen();
    dbg!(x);

    let mut nums: Vec<i32> = (0..10).collect();
    nums.shuffle(&mut rng);
    dbg!(nums);

    dbg!(roll_d6(&mut rng));
}

//fn roll_d6<R: Rng>(rng: &mut R) -> u8 {
//    (1..=6).choose(rng).unwrap()
//}

fn roll_d6<R: Rng>(rng: &mut R) -> u8 {
    (1..=6).choose(rng).unwrap()
}
