fn main() {
    dbg!(1 < 2);
    dbg!('a' < 'f');
    dbg!((1, 'f') < (2, 'a'));
    dbg!([1, 2] < [2, 1]);
    dbg!(vec![1, 2] < vec![2, 1]);
}
