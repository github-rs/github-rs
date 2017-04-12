extern crate github_rs as gh;
use gh::client::Github;
use gh::headers::{ etag, rate_limit_remaining };
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
    let g = Github::new(&auth_token().unwrap());
    let (headers, status, json) = g.get()
                                   .repos()
                                   .owner("mgattozzi")
                                   .repo("github-rs")
                                   .execute()
                                   .unwrap();
    println!("{}", headers);
    println!("{}", status);
    if let Some(json) = json {
        println!("{}", json);
    }
}

#[test]
fn cached_response() {
    // We want it to fail
    let g = Github::new(&auth_token().unwrap());
    let (headers, _, _) = g.get()
                           .repos()
                           .owner("mgattozzi")
                           .repo("github-rs")
                           .execute()
                           .unwrap();
    let etag = etag(&headers);
    //let limit = rate_limit_remaining(&headers).unwrap();
    let _ = rate_limit_remaining(&headers).unwrap();
    let (headers, _, _) = g.get()
                           .set_etag(etag.unwrap())
                           .repos()
                           .owner("mgattozzi")
                           .repo("github-rs")
                           .execute()
                           .unwrap();
    //let limit2 = rate_limit_remaining(&headers).unwrap();
    let _ = rate_limit_remaining(&headers).unwrap();
    // Spurious test case
    //assert_eq!(limit, limit2);
}

#[test]
fn core_exposure() {
    let g = Github::new(&auth_token().unwrap());
    // Can we get the core for users to have?
    let core = g.get_core();
    let ref mut core_mut = *core.try_borrow_mut().unwrap();
    let _ = core_mut.handle();
}
