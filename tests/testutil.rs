extern crate github_rs as gh;

use gh::client::Github;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

const INVALID_TOKEN_FILE: &'static str = "Your auth_token file is not setup properly. \
Refer to the contributing guidelines to see how to set one up.";

pub const FAILED_GITHUB_CONNECTION: &'static str = "Unable to connect with GitHub. \
Make sure you have configured your access token correctly.";

fn auth_token() -> Result<String, Error> {
    let file = File::open("tests/auth_token")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn setup_github_connection() -> Github {
    Github::new(&auth_token().expect(INVALID_TOKEN_FILE)).unwrap()
}
