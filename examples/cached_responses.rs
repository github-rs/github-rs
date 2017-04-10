extern crate github_rs;
use github_rs::client::Github;
use github_rs::headers::{ etag, rate_limit_remaining };

fn main() {
    let client = Github::new("Your Auth Token Here");
    let me = client.get()
                   .user()
                   .execute();
    match me {
        Ok((headers, _, _)) => {

            if let Some(etag) = etag(&headers) {
                let limit = rate_limit_remaining(&headers);
                let (headers, _, _) = client.get()
                                            .set_etag(etag)
                                            .user()
                                            .execute()
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
