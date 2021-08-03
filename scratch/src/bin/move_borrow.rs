#[derive(Debug)]
struct Thing(bool);

fn main() {
    let x = make_thing();
    borrow_thing(&x);
    let mut y = transform_thing(x);
    borrow_thing(&y);
    change_thing(&mut y);
    move_thing(y);
}

fn make_thing() -> Thing {
    Thing(false)
}

fn change_thing(x: &mut Thing) {
    // let Thing(ref mut value) = x;
    // *value = true;
    x.0 = false;
}

fn transform_thing(x: Thing) -> Thing {
    // let Thing(value) = x;
    // Thing(!value)
    Thing(!x.0)
}

fn move_thing(x: Thing) {
    println!("{:?}", x);
}

fn borrow_thing(x: &Thing) {
    println!("{:?}", *x);
}
