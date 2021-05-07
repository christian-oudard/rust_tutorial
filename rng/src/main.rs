use rand::{rngs::StdRng, SeedableRng};

fn main() {
    let mut rng: StdRng = SeedableRng::from_seed([0]);

    let mut arr = [0i8; 20];
    rng.fill(&mut arr[..]);

    println!("{:?}", arr);

}
