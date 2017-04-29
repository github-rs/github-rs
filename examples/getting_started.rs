extern crate tokio_core;
extern crate github_rs;
use tokio_core::reactor::Core;
use github_rs::client::Github;

fn main() {
    let mut core = Core::new().unwrap();
    let client = Github::new("API TOKEN", &core.handle());
    let me = core.run(client.get()
                            .user()
                            .execute());
    match me {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}
