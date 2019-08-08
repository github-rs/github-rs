use github_rs::client::Executor;
use github_rs::StatusCode;

use serde_json::Value;

mod testutil;

use testutil::*;

fn get_rust_repos() -> Vec<String> {
    let mut list = Vec::new();
    let vec = setup_github_connection()
            .get()
            .orgs()
            .org("rust-lang")
            .repos()
            .execute_all_pages::<Value>()
            .expect(testutil::FAILED_GITHUB_CONNECTION);
    for page in vec {
        if let (_headers, StatusCode::OK, Value::Array(repos)) = page {
            list.extend(repos.into_iter().map(|o| o["name"].as_str().unwrap().into()));
        }
    }
    list
}

#[test]
fn list_rust_repos() {
    println!("{:#?}", get_rust_repos());
}
