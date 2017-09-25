use IntoGithubRequest;
use hyper::{ Uri, Method, Request };
use hyper::header::{ Authorization, ContentType, UserAgent };
use errors::*;
use std::str::FromStr;

/// Used to query information from the GitHub API to possibly be used in
/// a `Mutation` or for information to make decisions with how to interact.
pub struct Query {
    pub(crate) query: String,
}

impl Query {
    /// Create a new `Query`
    pub fn new() -> Self {
        Self { query: String::new() }
    }
    /// Create a new `Query` using the given value as the input for the query to
    /// GitHub. Any other methods used will assume the `String` is empty. This
    /// is a shortcut for doing:
    ///
    /// ```no_test
    /// let q = Query::new();
    /// q.raw_query("my query which won't work");
    /// ```
    ///
    /// as
    ///
    /// ```no_test
    /// let q = Query::new_raw("my query which won't work");
    /// ```
    pub fn new_raw<T>(q: T) -> Self
        where T: ToString
    {
        Self { query: q.to_string() }
    }

    /// Whatever you put here becomes your query and replaces anything you might
    /// have built up before. This assumes you know what you're doing with the
    /// API so no guarantees can be made here that it will work, only that if
    /// used this can be used to make a query using the `client::Github` type.
    pub fn raw_query<T>(&mut self, q: T)
        where T: ToString
    {
        self.query = q.to_string();
    }
}

impl IntoGithubRequest for Query {
    fn into_github_req(&self, token: &str) -> Result<Request> {
            let mut req = Request::new(
                Method::Post,
                Uri::from_str("https://api.github.com/graphql")
                    .chain_err(|| "Unable to for URL to make the request")?);
            let mut q = String::from("{ \"query\": \"");
            q.push_str(&self.query);
            q.push_str("\" }");
            req.set_body(q);
            let token = String::from("token ") + &token;
            {
                let headers = req.headers_mut();
                headers.set(ContentType::json());
                headers.set(UserAgent::new(String::from("github-rs")));
                headers.set(Authorization(token));
            }
            Ok(req)
    }
}
