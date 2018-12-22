use IntoGithubRequest;
use hyper::{ Request };
use hyper::header::{ HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT };
use errors::*;
use std::str::FromStr;

/// Used to mutate information on GitHub
pub struct Mutation {
    pub(crate) mutation: String,
}

impl Mutation {
    /// Create a new `Mutation`
    pub fn new() -> Self {
        Self { mutation: String::new() }
    }
    /// Create a new `Mutation` using the given value as the input for the query
    /// to GitHub. Any other methods used will assume the `String` is empty.
    /// This is a shortcut for doing:
    ///
    /// ```no_test
    /// let m = Mutation::new();
    /// m.raw_query("my query which won't work");
    /// ```
    ///
    /// as
    ///
    /// ```no_test
    /// let m = Mutation::new_raw("my query which won't work");
    /// ```
    pub fn new_raw<T>(m: T) -> Self
        where T: ToString
    {
        Self { mutation: m.to_string() }
    }

    /// Whatever you put here becomes your query and replaces anything you might
    /// have built up before. This assumes you know what you're doing with the
    /// API so no guarantees can be made here that it will work, only that if
    /// used this can be used to make a query using the `client::Github` type.
    pub fn raw_mutation<T>(&mut self, m: T)
        where T: ToString
    {
        self.mutation = m.to_string();
    }
}

impl IntoGithubRequest for Mutation {
    fn into_github_req(&self, token: &str) -> Result<Request<hyper::Body>> {
            let mut q = String::from("{ \"query\": \"");
            q.push_str(&self.mutation);
            q.push_str("\" }");
            println!("{}", q);
            let mut req = Request::builder()
                .method("POST")
                .uri("https://api.github.com/graphql")
                .body(q.into())
                .chain_err(|| "Unable to for URL to make the request")?;
            let token = String::from("token ") + &token;
            {
                let headers = req.headers_mut();
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers.insert(USER_AGENT, HeaderValue::from_static("github-rs"));
                headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).chain_err(|| "token parse")?);
            }
            Ok(req)
    }
}
