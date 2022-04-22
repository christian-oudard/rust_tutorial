use serde::{Deserialize, Serialize};
use serde_json::{
    Result, Value,
    ser::to_string_pretty,
};

fn main() {
    untyped_example().expect("serde error");
    typed_example().expect("serde error");
}

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let mut v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {}, aged {}, at the number {}", v["name"].as_str().expect("name not a string"), v["age"], v["phones"][0]);

    v.as_object_mut().unwrap().remove("age");
    println!("{}", to_string_pretty(&v)?);

    Ok(())
}


#[derive(Serialize, Deserialize)]
struct Response {
    result: Option<RPCResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<RPCError>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RPCResult(String);
#[derive(Serialize, Deserialize, Debug)]
struct RPCError(String);


fn typed_example() -> Result<()> {
    let response: Response = serde_json::from_str(r#"
        {
            "result": null,
            "error": "error!"
        }
    "#)?;

    // Do things just like with any other Rust data structure.
    println!("result: {:?}", response.result);
    println!("{}", to_string_pretty(&response)?);

    let response: Response = serde_json::from_str(r#"
        {
            "error": null
        }
    "#)?;
    println!("{}", to_string_pretty(&response)?);

    Ok(())
}
