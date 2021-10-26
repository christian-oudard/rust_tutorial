
type Result = std::result::Result<String, ()>;

fn main() {
    println!("{:?}", maybe_parse_int(None));
    println!("{:?}", maybe_parse_int(Some("123".into())));
    println!("{:?}", maybe_parse_int(Some("invalid".into())));

    println!();
    let res_ok: Result = Ok("stuff".into());
    let res_err: Result = Err(());
    println!("{:?}", res_ok.ok());
    println!("{:?}", res_err.ok());

    println!();
    let res_ok: Result = Ok("stuff".into());
    let res_err: Result = Err(());
    println!("{:?}", res_ok.err());
    println!("{:?}", res_err.err());
}

fn maybe_parse_int(maybe_s: Option<String>) -> Option<i64> {
    match maybe_s {
        Some(s) => s.parse().ok(),
        None => None,
    }
}
