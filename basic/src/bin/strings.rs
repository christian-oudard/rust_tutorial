fn main() {
    let s1: &str = "test";
    let s2: String = String::from("test");
    let s3: &String = &"test".into();
    let s4: String = "test".to_string();
    let s5: String = "test".to_owned();

    dbg!((&s1, &s2, &s3, &s4, &s5));
    dbg!(s1 == s2);
    dbg!(s1 == &s2);
    dbg!(&s1 == &s2);
    // dbg!(&s1 == s2); // Error
    dbg!(s1 == s3);

    f1(s1);
    // f1(s2); // Error
    f1(&s2);
    f1(s3);
    f1(&s1);
    f1(&&s1);
    f1(&&s2);
    f1(&&&&&s1);

    // f2(s1); // Error
    // f2(&s1); // Error
    f2(&s2);
    f2(s3);
    f2(&s3);
}

fn f1(s: &str) {
    println!("{}", s);
}

fn f2(s: &String) {
    println!("{}", s);
}
