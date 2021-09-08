fn main() {
    let v: Vec<u8> = vec![0b01111111; 1 << 17];
    println!("{}", v[123456]);
}
