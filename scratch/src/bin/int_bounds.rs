use core::mem;

macro_rules! check{ 
    ($t:ty)=>{
        dbg!(mem::size_of::<$t>());
        dbg!((<$t>::MIN, <$t>::MAX));
    }
}

fn main() {
    check!(u8);
    check!(i8);
    check!(u16);
    check!(i16);
    check!(u32);
    check!(i32);
    check!(u64);
    check!(i64);
    check!(u128);
    check!(i128);
    check!(usize);

    assert_eq!(mean(240, 250), 245);
    assert_eq!(mean(241, 249), 245);
    assert_eq!(mean(241, 250), 245);
    assert_eq!(mean(240, 251), 245);
}

/// Respect overflow while computing the mean of two values.
fn mean(a: u8, b:u8) -> u8 {
    a / 2 + b / 2 + (a & b & 1)
}
