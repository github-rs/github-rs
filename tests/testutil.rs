extern crate github_rs as gh;

use gh::client::Github;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

const INVALID_TOKEN_FILE: &'static str = "Auth token file is not configured correctly. \
Please refer contributing guidelines to see how to setup one.";

const FAILED_GITHUB_CONNECTION: &'static str = "Unable to connect with GitHub. \
Make sure you have configured access token correctly.";

fn auth_token() -> Result<String, Error> {
    let file = File::open("tests/auth_token")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn setup_github_connection() -> Github {
    Github::new(&auth_token().expect(INVALID_TOKEN_FILE)).expect(FAILED_GITHUB_CONNECTION)
}
