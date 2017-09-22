extern crate github_rs;
extern crate serde_json;
use github_rs::client::{ Executor, Github };
use serde_json::Value;

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
