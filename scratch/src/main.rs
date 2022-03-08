fn main() {
    let mut i: u64 = 0;
    for _ in 0..256 {
        println!("{:>20} {:>20} {:064b}", i, i as i64, i);
        i = i.wrapping_add(1<<56);
    }
}
