fn main() {
    // println!("{:?}", 10u8 - 12);
    println!("{:?}", 10u8.checked_sub(12));
    println!("{:?}", 10u8.saturating_sub(12));
    println!("{:?}", 10u8.overflowing_sub(12));
    println!("{:?}", 10u8.wrapping_sub(12));

    println!("");

    println!("{:?}", 10u8 - 2);
    println!("{:?}", 10u8.checked_sub(2));
    println!("{:?}", 10u8.saturating_sub(2));
    println!("{:?}", 10u8.overflowing_sub(2));
    println!("{:?}", 10u8.wrapping_sub(2));

    println!("");

    println!("{:?}", -10i8 as u8);
    println!("{:?}", 246u8 as i8);
}
