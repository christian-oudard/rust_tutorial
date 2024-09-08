const SIZE: usize = 100;

fn primes() -> Vec<u64> {
    let mut marks = vec![true; SIZE];
    marks[0] = false;
    marks[1] = false;
    for n in 0..SIZE {
        if marks[n] {
            let mut j = n + n;
            while j < SIZE {
                marks[j] = false;
                j += n;
            }
        }
    }

    return marks.into_iter().enumerate().filter_map(
        |(i, m)| if m {
            Some(i as u64)
        } else {
            None
        }
    ).collect();
}

fn main() {
    for p in primes() {
        println!("{}", p);
    }
}
