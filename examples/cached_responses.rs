extern crate github_rs;
extern crate serde_json;
use github_rs::client::{ Executor, Github };
use github_rs::headers::{ etag, rate_limit_remaining };
use serde_json::Value;

fn main() {
    let client = Github::new("Your Auth Token Here").unwrap();
    let me = client.get()
                   .user()
                   .execute::<Value>();
    match me {
        Ok((headers, _, _)) => {

            if let Some(etag) = etag(&headers) {
                let limit = rate_limit_remaining(&headers);
                let (headers, _, _) = client.get()
                                            .set_etag(etag)
                                            .user()
                                            .execute::<Value>()
                                            .expect("Well I existed before");
                if let Some(limit) = limit {
                    println!("Asserting they are equal!");
                    assert_eq!(limit, rate_limit_remaining(&headers).unwrap());
                    println!("They are!");
                }
            }
        },
        Err(e) => println!("{}", e)
    }
}
