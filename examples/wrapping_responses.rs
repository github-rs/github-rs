extern crate github_rs;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use github_rs::client::{Executor, Github};
use github_rs::StatusCode;
use serde_json::Value;

trait TryExecute: Executor {
    fn try_execute(self) -> Result<Value, String>
    where
        Self: Sized,
    {
        #[derive(Deserialize)]
        struct GithubError {
            message: String,
        }

        match self.execute::<Value>() {
            Ok((_, StatusCode::OK, Some(response))) => Ok(response),
            Ok((_, _, Some(response))) => serde_json::from_value::<GithubError>(response)
                .map_err(|err| format!("Failed to parse error response: {}", err))
                .and_then(|error| Err(error.message.into())),
            Ok((_, _, None)) => Err("Received error response from github with no message".into()),
            Err(err) => Err(format!("Failed to execute request: {}", err)),
        }
    }
}

impl<'a> TryExecute for ::github_rs::users::get::User<'a> {}

fn main() {
    let client = Github::new("API TOKEN").unwrap();
    let result = client.get().user().try_execute();
    match result {
        Ok(me) => println!("{}", me),
        Err(err) => println!("{}", err),
    }
}
