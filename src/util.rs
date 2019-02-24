use hyper::http::uri::InvalidUriParts;
use hyper::http::HttpTryFrom;
use hyper::Uri;

/// Add an extra subdirectory to the end of the url. This utilizes
/// Hyper's more generic Uri type. We've set it up to act as a Url.
pub fn url_join(url: &Uri, path: &str) -> Result<Uri, InvalidUriParts> {
    let mut parts = url.clone().into_parts();
    let p = parts.path_and_query.take();
    let curr_path = match p {
        Some(ref p) => p.path(),
        None => "",
    };
    let mut curr_path = String::from(curr_path);
    if !curr_path.ends_with('/') {
        curr_path.push('/');
    }
    curr_path.push_str(path);
    parts.path_and_query = HttpTryFrom::try_from(curr_path.as_str()).ok();
    Uri::from_parts(parts)
}
