fn main() {
    let bytes: Vec<u8> = vec![0xFF];
    std::fs::write("invalid", bytes).unwrap();
}
