use cached::proc_macro::cached;

fn main() {
    println!("{}", fib_iter(30));
    println!("{}", fib(30));
}

#[cached]
fn fib(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

fn fib_iter(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut a, mut b) = (0, 1);
    let mut c: u32;
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}
