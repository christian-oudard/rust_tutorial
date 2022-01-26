use rayon::prelude::*;
use std::time::{Duration, Instant};

const P: u64 = 1692619087;
const Q: u64 = 399382639;
const M: u64 = P * Q;

fn bbs(prev: u64, m: u64) -> u64 {
    // https://en.wikipedia.org/wiki/Blum_Blum_Shub
    let x = prev as u128;
    let x2 = x * x;
    (x2 % m as u128) as u64
}

fn workload() -> u64 {
    let mut x = 2;
    for _ in (0..50_000_000) {
        x = bbs(x, M);
    }
    x
}

fn main() {
    let start = Instant::now();
    let results: Vec<u64> = (0..16).into_par_iter().map(|_| workload()).collect();
    let duration = start.elapsed();
    for x in results {
        println!("{:016x}", x);
    }
    println!("{:?}", duration);
}



