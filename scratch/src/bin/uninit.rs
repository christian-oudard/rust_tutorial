
use std::mem::MaybeUninit;

#[allow(unreachable_code)]
fn main() {
    let x = MaybeUninit::<u16>::uninit();
    let x = unsafe { x.assume_init() };
    dbg!(x);
    g();
    f(); 
}

fn f() -> ! {
    panic!("test")
}

fn g() -> ! {
    loop { }
}


