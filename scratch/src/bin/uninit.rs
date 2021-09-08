use std::mem::MaybeUninit;

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


