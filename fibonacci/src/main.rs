use cached::proc_macro::cached;

fn main() {
    println!("{}", fib_iter(50));
    println!("{}", fib(50));
}

#[cached]
fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}

fn fib_iter(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut a, mut b) = (0, 1);
    let mut c: u64;
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}


// struct Fibonacci<T> {
//     cur: T,
//     last: T,
// }

// impl<T> Fibonacci<T> {
//     fn new() -> Fibonacci<T> {
//         Fibonacci { last: 0, cur: 1 }
//     }
// }

// impl Iterator for Fibonacci {
//     type Item = u64;
//     fn next(&mut self) -> Option<Self::Item> {
//     }
// }


