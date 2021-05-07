use serde_json::json;

struct Thing

fn main() {
    let j = json!({"z": 1, "a": 2});
    println!("{}", j.to_string());
}
