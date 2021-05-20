fn main() {
    let s = "Isn't it grand?";
    println!("{}", s);
    println!("{}", sql_escape_string(s.to_string()));
}

/// Escape a string for consumption by SQLite.
/// This function doubles all single quote characters within the string, then wraps the string in
/// single quotes on the front and back.
fn sql_escape_string(s: String) -> String {
    format!("'{}'", s.replace("'", "''"))
}

fn sql_escape_string_2(s: String) -> String {
    let mut s_escaped = String::new();
    for c in s.chars() {
        if c == '\'' {
            s_escaped.push_str("''");
        } else {
            s_escaped.push(c);
        }
    }
    return format!("'{}'", s_escaped);
}
