use reqwest::{self, header::CONTENT_TYPE};

fn main() -> Result<(), reqwest::Error> {
    env_logger::init();

    println!("POST");

    let client = reqwest::blocking::Client::new();
    let request_json = r#"
        {
            "method": "get_all_accounts",
            "jsonrpc": "2.0",
            "api_version": "2",
            "id": 1
        }
    "#;
    let res = client
        .post("http://127.0.0.1:9090/wallet")
        .header(CONTENT_TYPE, "application/json")
        .body(request_json)
        .send()?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    println!("{}", res.text()?);

    println!("\n\nDone.");
    Ok(())
}
