use bigdecimal::BigDecimal;
use std::str::FromStr;

fn main() {
    let two = BigDecimal::from(2);
    let sqrt_two = two.sqrt().unwrap();
    dbg!(sqrt_two);

    //let pi = BigDecimal::from(3.14159265359); // No.
    let pi = BigDecimal::from_str("3.14159265359").unwrap();
    dbg!(pi);

}

