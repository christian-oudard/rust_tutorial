fn main() {
    let s = "abcdefg";
    let v: Vec<_> = s.chars().collect();
    for w in v.windows(3) {
        dbg!(w);
    }
}
