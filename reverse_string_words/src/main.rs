// Reverse the order of words in a string, in-place.
fn main() {
    let s = "Reverse the order of these words.";
    let mut v: Vec<char> = s.chars().collect();
    reverse_word_order(&mut v);
    let s: String = v.into_iter().collect();
    println!("{}", s);
}

fn reverse(v: &mut [char]) {
    for i in 0..(v.len() / 2) {
        v.swap(i, v.len() - 1 - i);
    }
}

fn reverse_word_order(v: &mut [char]) {
    let n = v.len();

    // Start by reversing the whole string.
    reverse(&mut v[0..n]);

    // Un-reverse each word within the string,
    let mut word_start = 0;
    for i in 0..n {
        if v[i] == ' ' {
            reverse(&mut v[word_start..i]);
            word_start = i + 1;
        }
    }
    reverse(&mut v[word_start..n]);
}
