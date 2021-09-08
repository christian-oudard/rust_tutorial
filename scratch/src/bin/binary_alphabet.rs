// use std::char;
// use std::collections::HashMap;

// struct Alphabet {
//     letters: Vec<u8>,
//     lookup: HashMap<u8, u8>,
// }

// impl Alphabet {
//     fn new(letters: &[u8]) -> Alphabet {
//         Alphabet {
//             letters: Vec::from(letters),
//             lookup: letters
//                 .iter()
//                 .enumerate()
//                 .map(|(i, c)| (*c, i as u8))
//                 .collect(),
//         }
//     }

//     fn number_to_letter(&self, i: u8) -> u8 {
//         self.letters[i as usize]
//     }

//     fn letter_to_number(&self, c: u8) -> u8 {
//         *self.lookup.get(&c).unwrap()
//     }

//     fn string_to_binary(&self, s: &str) -> {
//         let mut result = Vec::new();
//         for c in s {
//             result.extend(number_to_binary(letter_to_number(c)))
//         }
//     }
// }

// fn number_to_binary(x: u8, digits: u8) -> String {

// }

fn main() {
//     Alphabet::new(b" ABCDEFGHIJKLMNOPQRSTUVWXYZ.,;!?");
//     println!("{:b}", 'C' as u32);
}
