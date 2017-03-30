extern crate github_rs;
use github_rs::client::Github;

fn main() {
    let mut client = Github::new("API TOKEN");
    let me = client.get()
                   .user()
                   .execute();
    match me {
        Ok((status, json)) => {
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}
