use base64::decode;

fn main() {
    let bytes = decode("S1JZUFRPTklTR1JFQVQ=").unwrap();
    let s = std::str::from_utf8(&bytes);
    dbg!(s).unwrap();
}
