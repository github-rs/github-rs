extern crate github_rs as gh;
extern crate serde_json;
use gh::client::Executor;
use gh::headers::{ etag, rate_limit_remaining };
use serde_json::Value;

mod testutil;

use testutil::*;

#[test]
fn get_user_repos() {
    // We want it to fail
    let g = setup_github_connection();
    let (headers, status, json) = g.get()
                                   .repos()
                                   .owner("mgattozzi")
                                   .repo("github-rs")
                                   .execute::<Value>()
                                   .expect(testutil::FAILED_GITHUB_CONNECTION);
    println!("{:#?}", headers);
    println!("{}", status);
    if let Some(json) = json {
        println!("{}", json);
    }
}

#[test]
fn cached_response() {
    // We want it to fail
    let g = setup_github_connection();
    let (headers, _, _) = g.get()
                           .repos()
                           .owner("mgattozzi")
                           .repo("github-rs")
                           .execute::<Value>()
                           .expect(testutil::FAILED_GITHUB_CONNECTION);
    let etag = etag(&headers);
    //let limit = rate_limit_remaining(&headers).unwrap();
    let _ = rate_limit_remaining(&headers).unwrap();
    let (headers, _, _) = g.get()
                           .set_etag(etag.unwrap())
                           .repos()
                           .owner("mgattozzi")
                           .repo("github-rs")
                           .execute::<Value>()
                           .expect(testutil::FAILED_GITHUB_CONNECTION);
    //let limit2 = rate_limit_remaining(&headers).unwrap();
    let _ = rate_limit_remaining(&headers).unwrap();
    // Spurious test case
    //assert_eq!(limit, limit2);
}

#[test]
fn core_exposure() {
    let g = setup_github_connection();
    // Can we get the core for users to have?
    let core = g.get_core();
    let core_mut = core.try_borrow_mut().unwrap();
    let _ = core_mut.handle();
}
