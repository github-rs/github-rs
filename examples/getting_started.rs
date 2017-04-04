extern crate github_rs;
use github_rs::client::Github;

fn main() {
    let client = Github::new("API TOKEN");
    let me = client.get()
                   .user()
                   .execute();
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
