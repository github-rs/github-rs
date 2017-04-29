extern crate tokio_core;
extern crate github_rs as gh;
use gh::client::Github;
use gh::headers::{ etag, rate_limit_remaining };
use tokio_core::reactor::Core;
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
    let mut core = Core::new().unwrap();
    let g = Github::new(&auth_token().unwrap(), &core.handle());
    let (headers, status, json) = core.run(g.get()
                                            .repos()
                                            .owner("mgattozzi")
                                            .repo("github-rs")
                                            .execute())
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
    let mut core = Core::new().unwrap();
    let g = Github::new(&auth_token().unwrap(), &core.handle());
    let (headers, _, _) = core.run(g.get()
                                    .repos()
                                    .owner("mgattozzi")
                                    .repo("github-rs")
                                    .execute())
                              .unwrap();
    let etag = etag(&headers).unwrap();
    //let limit = rate_limit_remaining(&headers).unwrap();
    let _ = rate_limit_remaining(&headers).unwrap();
    let (headers, _, _) = core.run(g.get()
                                    .set_etag(etag)
                                    .repos()
                                    .owner("mgattozzi")
                                    .repo("github-rs")
                                    .execute())
                               .unwrap();
    //let limit2 = rate_limit_remaining(&headers).unwrap();
    let _ = rate_limit_remaining(&headers).unwrap();
    // Spurious test case
    //assert_eq!(limit, limit2);
}
