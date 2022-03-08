fn main() {
    let mut i: u8 = 0;
    for _ in 0..256 {
        println!("{:>3} {:>4} {:08b}", i, i as i8, i);
        i = i.wrapping_add(1);
    }
}
