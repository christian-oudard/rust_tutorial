fn main() {
    dbg!(10u8 - 2);
    dbg!(10u8.checked_sub(2));
    dbg!(10u8.saturating_sub(2));
    dbg!(10u8.overflowing_sub(2));
    dbg!(10u8.wrapping_sub(2));

    dbg!(10u8.checked_sub(12));
    dbg!(10u8.saturating_sub(12));
    dbg!(10u8.overflowing_sub(12));
    dbg!(10u8.wrapping_sub(12));

    #[allow(arithmetic_overflow)]
    {
        dbg!(10u8 - 12);
    }
}
