fn main() {
    let s = "test";
    let mut v: Vec<_> = s.chars().collect();
    v.sort();
    v.dedup();
    dbg!(&v);
    dbg!(&v.len());
}
