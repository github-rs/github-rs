use github_rs as gh;
#[macro_use]
extern crate serde_json;
use gh::client::Executor;
use gh::StatusCode;
use serde_json::Value;

mod testutil;

use testutil::*;

// These tests require you to have notifications on your profile. If you don't have a notification
// only the first test will be called. The tests won't fail if you don't have any notifications.

#[test]
fn get_notifications() {
    let g = setup_github_connection();
    let (headers, status, json) = g
        .get()
        .notifications()
        .execute::<Value>()
        .expect(testutil::FAILED_GITHUB_CONNECTION);
    println!("{:#?}", headers);
    println!("{}", status);
    println!("{:?}", json);
    assert_eq!(status, StatusCode::OK);
    if let Some(json) = json {
        if !json.as_array().unwrap().is_empty() {
            let id = json[0].get("id").unwrap().as_str().unwrap();
            get_single_thread_of_notifications(id);
            get_subscriptions_of_single_thread(id);
            put_subscriptions_for_a_thread(id);
            put_notifications(id);
        }
    }
}

fn get_single_thread_of_notifications(id: &str) {
    let g = setup_github_connection();
    let (headers, status, json) = g
        .get()
        .notifications()
        .threads()
        .id(id)
        .execute::<Value>()
        .expect(testutil::FAILED_GITHUB_CONNECTION);
    println!("{:#?}", headers);
    println!("{}", status);
    assert_eq!(status, StatusCode::OK);
    if let Some(json) = json {
        println!("{}", json);
    }
}

fn get_subscriptions_of_single_thread(id: &str) {
    let g = setup_github_connection();
    let (headers, status, json) = g
        .get()
        .notifications()
        .threads()
        .id(id)
        .subscription()
        .execute::<Value>()
        .expect(testutil::FAILED_GITHUB_CONNECTION);
    println!("{:#?}", headers);
    println!("{}", status);

    if let Some(json) = json {
        println!("{}", json);
    }
}

fn put_notifications(id: &str) {
    let g = setup_github_connection();
    let (headers, status, _) = g
        .put(json!({ "id": id }))
        .notifications()
        .execute::<Value>()
        .expect(testutil::FAILED_GITHUB_CONNECTION);
    println!("{:#?}", headers);
    println!("{}", status);
    assert_eq!(status, StatusCode::RESET_CONTENT);
}

fn put_subscriptions_for_a_thread(id: &str) {
    let g = setup_github_connection();
    let (headers, status, _) = g
        .put(json!({"subscribed": true, "ignored": false}))
        .notifications()
        .threads()
        .id(id)
        .subscription()
        .execute::<Value>()
        .expect(testutil::FAILED_GITHUB_CONNECTION);
    println!("{:#?}", headers);
    println!("{}", status);
    assert_eq!(status, StatusCode::OK);
}
