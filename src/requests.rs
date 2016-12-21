//! Library calls to generate and manipulate requests to the
//! Github API

use hyper::header::{Accept, UserAgent, Authorization, Headers, qitem};
use hyper::mime::Mime;
use hyper::client::Client;
use hyper::status::StatusCode;
use types::*;
use error::*;
use std::io::Read;

// Notes on these comments: https://developer.github.com/v3/
// Implement Conditional requests to reduce API calls w/ ETag headers
// Add ability to allow CORS if requested.

/// Various Media Types that can be used to access the Github API
/// by default it just uses Json but various endpoints can return
/// data differently based off the Media Type. Care should be used
/// as some methods will fail given parsing to specific types.
#[derive(Debug,Copy,Clone)]
pub enum MediaType {
    /// Get JSON back from the API
    Json,
    /// Get Raw JSON back from the API
    RawJson,
    /// Get Text in JSON form back from the API
    TextJson,
    /// Get HTML in JSON form back from the API
    HtmlJson,
    /// Get Full in JSON form back from the API
    FullJson,
    /// Get Diffs back from the API
    Diff,
    /// Get Patches back from the API
    Patch,
    /// Get Raw data back from the API
    Raw,
    /// Get SHA back from the API
    Sha,
    /// Get HTML back from the API
    Html,
    /// Get Base64 back from the API
    Base64
}

/// Change a MediaType into it's coresponding Mime type for requests
pub fn media_to_mime(media: MediaType) -> Result<Mime> {
    match media {
        // try_mime unwraps the value like try but converts to a github
        // error otherwise however since it unwraps if it's okay we need to
        // wrap it in an Ok if it is successful
        MediaType::Json     => Ok(try_mime!("application/vnd.github.v3+json".parse())),
        MediaType::RawJson  => Ok(try_mime!("application/vnd.github.v3.raw+json".parse())),
        MediaType::TextJson => Ok(try_mime!("application/vnd.github.v3.text+json".parse())),
        MediaType::HtmlJson => Ok(try_mime!("application/vnd.github.v3+html+json".parse())),
        MediaType::FullJson => Ok(try_mime!("application/vnd.github.v3.full+json".parse())),
        MediaType::Diff     => Ok(try_mime!("application/vnd.github.v3.diff".parse())),
        MediaType::Patch    => Ok(try_mime!("application/vnd.github.v3.patch".parse())),
        MediaType::Raw      => Ok(try_mime!("application/vnd.github.v3.raw".parse())),
        MediaType::Sha      => Ok(try_mime!("application/vnd.github.v3.sha".parse())),
        MediaType::Html     => Ok(try_mime!("application/vnd.github.v3.html".parse())),
        MediaType::Base64   => Ok(try_mime!("application/vnd.github.v3.base64".parse())),
    }
}

/// Default Headers returns a list with the user agent and `AccessToken`
/// already in a list for use in a request to the API
pub fn default_headers(auth_token: AccessToken) -> Result<Headers> {
    // Used to make sure we always request the v3 version of the API
    // so as to make these bindings stable
    let mime: Mime = try_mime!("application/vnd.github.v3+json".parse());
    let token = String::from("token ") + &auth_token;

    let mut headers = Headers::new();
    headers.set(UserAgent(String::from("github-rs")));
    headers.set(Authorization(token));
    headers.set(Accept(vec![qitem(mime)]));

    Ok(headers)
}

/// Allows easy pagination by manipulating the url
pub fn pagination(url: &mut String, page: Option<u64>, num_per: Option<u64>) {
    if let Some(page_num) = page {
        url.push_str("?page=");
        url.push_str(&page_num.to_string());
    }
    if let Some(num) = num_per {
        url.push_str("&per_page=");
        url.push_str(&num.to_string());
    }
}

/// GET requests
pub fn get(url: &str, input_headers: Headers) -> Result<RawJSON> {
    let client = Client::new();
    let request = client.get(url);
    let mut buffer = String::new();

    // Send a request and read out the JSON response into a buffer
    let mut res = try_hyper!(request.headers(input_headers).send());
    if let Err(err) = res.read_to_string(&mut buffer) {
        Err(GithubError::LibIo(err))
    } else {
        Ok(buffer)
    }

}

/// GET request with only the status code returned
pub fn get_status_code(url: &str, input_headers: Headers) -> Result<StatusCode> {
    let client = Client::new();
    let request = client.get(url);

    // Send a request and read out the JSON response into a buffer
    let res = try_hyper!(request.headers(input_headers).send());
    Ok(res.status)
}

/// PUT requests
/// Github doesn't use PUT to update things. It uses PATCH to do that.
/// PUT requests all return a `StatusCode` to be used for verification
/// that things went right
pub fn put(url: &str, input_headers: Headers) -> Result<StatusCode> {

    let client = Client::new();
    let request = client.put(url);

    // Send a request and read out the JSON response into a buffer
    let res = try_hyper!(request.headers(input_headers).send());
    Ok(res.status)

}

/// POST requests
pub fn post(url: &str, input_headers: Headers, json: String) -> Result<RawJSON> {

    let client = Client::new();
    let request = client.post(url).body(json.as_bytes());
    let mut buffer = String::new();

    // Send a request and read out the JSON response into a buffer
    let mut res = try_hyper!(request.headers(input_headers).send());
    if let Err(err) = res.read_to_string(&mut buffer) {
        Err(GithubError::LibIo(err))
    } else {
        Ok(buffer)
    }
}

/// DELETE requests all return a `StatusCode` to be used for verification
/// that things went right
pub fn delete(url: &str, input_headers: Headers) -> Result<StatusCode> {

    let client = Client::new();
    let request = client.delete(url);

    // Send a request and read out the JSON response into a buffer
    let res = try_hyper!(request.headers(input_headers).send());
    Ok(res.status)

}

/// DELETE requests with specific data to remove
pub fn delete_with_data(url: &str, input_headers: Headers, json: String) -> Result<StatusCode> {

    let client = Client::new();
    let request = client.delete(url).body(json.as_bytes());

    // Send a request and read out the JSON response into a buffer
    let res = try_hyper!(request.headers(input_headers).send());
    Ok(res.status)

}

/// PATCH requests
pub fn patch(url: &str, input_headers: Headers, json: String) -> Result<RawJSON> {
    let client = Client::new();
    let request = client.patch(url).body(json.as_bytes());
    let mut buffer = String::new();

    // Send a request and read out the JSON response into a buffer
    let mut res = try_hyper!(request.headers(input_headers).send());
    if let Err(err) = res.read_to_string(&mut buffer) {
        Err(GithubError::LibIo(err))
    } else {
        Ok(buffer)
    }
}
