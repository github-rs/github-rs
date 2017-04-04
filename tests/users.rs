extern crate github_rs as gh;
use gh::client::Github;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn auth_token() -> Result<String, std::io::Error> {
    let file = File::open("tests/auth_token")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer)?;
    Ok(buffer)
}

#[test]
fn get_user_repos() {
    // We want it to fail
    let mut g = Github::new(&auth_token().unwrap());
    let (status, json) = g.get()
                          .repos()
                          .owner("mgattozzi")
                          .repo("github-rs")
                          .execute()
                          .unwrap();
    println!("{}", status);
    if let Some(json) = json {
        println!("{}", json);
    }
}
