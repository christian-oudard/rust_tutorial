use async_std::task::sleep;
use futures::executor::block_on;
use futures::future::FutureExt;
use rand::{thread_rng, Rng};
use std::time::Duration;

async fn sleep_random(lo: u64, hi: u64) {
    let mut rng = thread_rng();
    let ms = rng.gen_range(lo..=hi);
    sleep(Duration::from_millis(ms)).await;
    println!("slept {}ms", ms);
}

async fn hello_world(i: u32) {
    sleep_random(10, 50).await;
    println!("hello, world {}!", i);
}

async fn async_main() {
    let f1 = hello_world(1);
    let f2 = hello_world(2);
    futures::join!(f1, f2);

    let f = hello_world(42);
    f.await;

    let future = async { 6 };
    let shared1 = future.shared();
    let shared2 = shared1.clone();

    println!("{}", shared1.await);
    println!("{}", shared2.await);
}

fn main() {
    block_on(async_main());
    block_on(async { println!("async block") });
}
