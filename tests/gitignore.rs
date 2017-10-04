extern crate github_rs as gh;
extern crate serde_json;
use gh::StatusCode;
use gh::client::{Executor, Github};
use serde_json::Value;
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
fn get_gitignore_templates() {
    let g = Github::new(&auth_token().unwrap()).unwrap();
    let (_headers, status, json) = g.get().gitignore().templates().execute::<Value>().unwrap();
    println!("Status: {}\nResponse: {:#?}", status, json);
    assert_eq!(status, StatusCode::Ok);
    if let Some(Value::Array(languages)) = json {
        assert!(languages.contains(&Value::String("Rust".into())))
    }
}

#[test]
fn get_gitignore_templates_rust() {
    let g = Github::new(&auth_token().unwrap()).unwrap();
    let (_headers, status, json) = g.get()
        .gitignore()
        .templates()
        .lang("Rust")
        .execute::<Value>()
        .unwrap();
    println!("Status: {}\nResponse: {:#?}", status, json);
    assert_eq!(status, StatusCode::Ok);
    if let Some(json) = json {
        assert!(
            json.get("source")
                .unwrap()
                .as_str()
                .unwrap()
                .contains("Cargo")
        )
    }
}
