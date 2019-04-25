use github_rs as gh;

use gh::client::Github;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

const INVALID_TOKEN_FILE: &'static str =
    "Your auth_token file is not setup properly. \
     Refer to the contributing guidelines to see how to set one up.";

pub const FAILED_GITHUB_CONNECTION: &'static str =
    "Unable to connect with GitHub. \
     Make sure you have configured your access token correctly.";

fn auth_token() -> Result<String, Error> {
    let file = File::open("tests/auth_token")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    // Editors may add a newline at the end of a file, so this trims it off
    buffer = buffer.trim_end().to_string();
    Ok(buffer)
}

pub fn setup_github_connection() -> Github {
    Github::new(&auth_token().expect(INVALID_TOKEN_FILE)).unwrap()
}
