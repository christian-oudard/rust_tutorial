// Print a triangle of numbers.
fn main() {
    let mut i: u32 = 0;
    for y in 0..10 {
        let row: Vec<String> = (i..(i+y)).map(|i| i.to_string()).collect();
        println!("{}", row.join(" "));
        i += y;
    }
}
