extern crate serde_json;
use serde_json::Value;

extern crate github_rs;
use github_rs::client::Github;

fn main() {
    let client = Github::new("API TOKEN").unwrap();
    let me = client.get()
                   .user()
                   .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}
