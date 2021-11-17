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
}
