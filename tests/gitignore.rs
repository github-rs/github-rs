extern crate github_rs as gh;
extern crate serde_json;
use gh::StatusCode;
use gh::client::Executor;
use serde_json::Value;

mod testutil;

use testutil::*;

#[test]
fn get_gitignore_templates() {
    let g = setup_github_connection();
    let (_headers, status, json) = g.get()
                                    .gitignore()
                                    .templates()
                                    .execute::<Value>()
                                    .expect(testutil::FAILED_GITHUB_CONNECTION);
    println!("Status: {}\nResponse: {:#?}", status, json);
    assert_eq!(status, StatusCode::Ok);
    if let Some(Value::Array(languages)) = json {
        assert!(languages.contains(&Value::String("Rust".into())))
    }
}

#[test]
fn get_gitignore_templates_rust() {
    let g = setup_github_connection();
    let (_headers, status, json) = g.get()
        .gitignore()
        .templates()
        .lang("Rust")
        .execute::<Value>()
        .expect(testutil::FAILED_GITHUB_CONNECTION);
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
