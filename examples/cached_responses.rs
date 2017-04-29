extern crate tokio_core;
extern crate github_rs;
use tokio_core::reactor::Core;
use github_rs::client::Github;
use github_rs::headers::{ etag, rate_limit_remaining };

fn main() {
    let mut core = Core::new().unwrap();
    let client = Github::new("Your Auth Token Here", &core.handle());
    let me = core.run(client.get()
                            .user()
                            .execute());
    match me {
        Ok((headers, _, _)) => {

            if let Some(etag) = etag(&headers) {
                let limit = rate_limit_remaining(&headers);
                let me_again = core.run(client.get()
                                              .set_etag(etag)
                                              .user()
                                              .execute());
                let (headers, _, _) = me_again.expect("Well I existed before");
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
