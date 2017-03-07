use hyper::Uri;
use hyper::error::UriError;
use std::str::FromStr;

/// Work around function for Url.join() until it gets fixed upstream.
/// Adds the value passed in to the Url path
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
        // I think this will do it
        _ => Uri::from_str("fail"),
    }
}
