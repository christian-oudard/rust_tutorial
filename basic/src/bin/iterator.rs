fn main() {
    dbg!((0..5).into_iter().map(|x| x * 2).collect::<Vec<_>>());
    dbg!((0..10).into_iter().filter(|x| x % 2 == 1).collect::<Vec<_>>());

    let v = vec![None, Some(1), Some(2), None, Some(4)];
    dbg!(v.iter().filter(|x| x.is_some()).collect::<Vec<_>>());
    dbg!(v.iter().filter_map(|x| *x).collect::<Vec<_>>());

    dbg!(vec!["a", "b", "c"].into_iter().collect::<String>());
}
