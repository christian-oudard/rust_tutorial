use std::iter;

fn main() {
    for i in count() {
        print!("{} ", i);
    }
}

fn count() -> impl Iterator<Item = u8> {
    let mut i: u8 = 0;
    iter::from_fn(move || {
        i += 1;
        if i < u8::MAX {
            Some(i)
        } else {
            None
        }
    })

}
