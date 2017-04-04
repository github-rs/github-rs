use hyper::Uri;
use hyper::error::UriError;
use std::str::FromStr;

/// Add an extra subdirectory to the end of the url. This utilizes
/// Hyper's more generic Uri type. We've set it up to act as a Url.
pub fn url_join(url: &Uri, path: &str) -> Result<Uri, UriError> {
    // Absolutely hackish but don't know anything better
    match (url.scheme(), url.authority(), url.path()) {
        (Some(s), Some(a), p) => {
            let mut curr_path = String::from(s);
            curr_path += "://";
            curr_path += a;
            curr_path += p;
            if !curr_path.ends_with('/') {
                curr_path.push('/');
            }
            curr_path.push_str(path);
            curr_path.parse::<Uri>()
        },
        // This should cause the request to fail if something goes
        // wrong.
        _ => Uri::from_str("Failed to make a valid Url"),
    }
}
