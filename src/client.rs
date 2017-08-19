// Hyper Imports
use hyper::header::{ Authorization, Accept, ContentType,
                     ETag, IfNoneMatch, UserAgent, qitem };
use hyper::mime::Mime;

// Reqwest Imports
use reqwest::header::Headers;
use reqwest::{ Client, Method, Request, StatusCode, Url };

// Serde Imports
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

// Internal Library Imports
use users;
use misc;
use repos;
use errors::*;

use std::io::Read;

/// Struct used to make calls to the Github API.
pub struct Github {
    token: String,
    client: Client,
}

impl Clone for Github {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            client: self.client.clone(),
        }
    }
}

/// All GET based queries can be constructed from this type
new_type!(GetQueryBuilder);

/// All PUT based queries can be constructed from this type
new_type!(PutQueryBuilder);

/// All POST based queries can be constructed from this type
new_type!(PostQueryBuilder);

/// All DELETE based queries can be constructed from this type
new_type!(DeleteQueryBuilder);

/// All PATCH based queries can be constructed from this type
new_type!(PatchQueryBuilder);

/// Queries for endpoints that aren't in this library can be crafted here
new_type!(CustomQuery);

/// This type is the final type used to execute a query
new_type!(Executor);


impl Github {
    /// Create a new Github client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API Token your requests will work.
    pub fn new<T>(token: T) -> Result<Self>
        where T: ToString {
        Ok(Self {
            token: token.to_string(),
            client: Client::new()?,
        })
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Change the currently set Authorization Token using a type that can turn
    /// into an &str. Must be a valid API Token for requests to work.
    pub fn set_token<T>(&mut self, token: T)
        where T: ToString {
        self.token = token.to_string();
    }

    /// Begin building up a GET request to GitHub
    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }

    /// Begin building up a PUT request with no data to GitHub
    pub fn put_empty(&self) -> PutQueryBuilder {
        self.into()
    }

    /// Begin building up a PUT request with data to GitHub
    pub fn put<T>(&self, body: T) -> PutQueryBuilder
        where T: Serialize {
        let mut qb: PutQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.body_mut() = Some(json.into());
                    qb.request = Ok(qbr);
                },
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }
        qb
    }

    /// Begin building up a POST request with data to GitHub
    pub fn post<T>(&self, body: T) -> PostQueryBuilder
        where T: Serialize {
        let mut qb: PostQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.body_mut() = Some(json.into());
                    qb.request = Ok(qbr);
                },
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }

        qb
    }

    /// Begin building up a PATCH request with data to GitHub
    pub fn patch<T>(&self, body: T) -> PatchQueryBuilder
        where T: Serialize {
        let mut qb: PatchQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.body_mut() = Some(json.into());
                    qb.request = Ok(qbr);
                },
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }
        qb
    }

    /// Begin building up a DELETE request with data to GitHub
    pub fn delete<T>(&self, body: T) -> DeleteQueryBuilder
        where T: Serialize {
        let mut qb: DeleteQueryBuilder = self.into();

        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.body_mut() = Some(json.into());
                    qb.request = Ok(qbr);
                },
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }
        qb
    }

    /// Begin building up a DELETE request without data to GitHub
    pub fn delete_empty(&self) -> DeleteQueryBuilder {
        self.into()
    }

}

impl <'g> GetQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    ///
    /// ```no_test
    /// # Don't have the beginning / in it
    /// repos/mgattozzi/github-rs
    /// ```
    ///
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);

    /// Query the emojis endpoint
    func_client!(emojis, misc::get::Emojis<'g>);

    /// Query the events endpoint
    func_client!(events, misc::get::Events<'g>);

    /// Query the feeds endpoint
    func_client!(feeds, misc::get::Feeds<'g>);

    /// Query the meta endpoint
    func_client!(meta, misc::get::Meta<'g>);

    /// Query the rate limit endpoint
    func_client!(rate_limit, misc::get::RateLimit<'g>);

    /// Query the user endpoint
    func_client!(user, users::get::User<'g>);

    /// Query the users endpoint
    func_client!(users, users::get::Users<'g>);

    /// Query the repos endpoint
    func_client!(repos, repos::get::Repos<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: ETag) -> Self {
        match self.request {
            Ok(mut req) => {
                let ETag(tag) = tag;
                req.headers_mut().set(IfNoneMatch::Items(vec![tag]));
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl <'g> PutQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    ///
    /// ```no_test
    /// # Don't have the beginning / in it
    /// repos/mgattozzi/github-rs
    /// ```
    ///
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::put::User<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: ETag) -> Self {
        match self.request {
            Ok(mut req) => {
                let ETag(tag) = tag;
                req.headers_mut().set(IfNoneMatch::Items(vec![tag]));
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl <'g> DeleteQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    ///
    /// ```no_test
    /// # Don't have the beginning / in it
    /// repos/mgattozzi/github-rs
    /// ```
    ///
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::delete::User<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: ETag) -> Self {
        match self.request {
            Ok(mut req) => {
                let ETag(tag) = tag;
                req.headers_mut().set(IfNoneMatch::Items(vec![tag]));
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl <'g> PostQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    ///
    /// ```no_test
    /// # Don't have the beginning / in it
    /// repos/mgattozzi/github-rs
    /// ```
    ///
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::post::User<'g>);
    func_client!(repos, repos::post::Repos<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: ETag) -> Self {
        match self.request {
            Ok(mut req) => {
                let ETag(tag) = tag;
                req.headers_mut().set(IfNoneMatch::Items(vec![tag]));
                self.request = Ok(req);
                self
            },
            Err(_) => self,
        }
    }
}

impl <'g> PatchQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    ///
    /// ```no_test
    /// # Don't have the beginning / in it
    /// repos/mgattozzi/github-rs
    /// ```
    ///
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::patch::User<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: ETag) -> Self {
        match self.request {
            Ok(mut req) => {
                let ETag(tag) = tag;
                req.headers_mut().set(IfNoneMatch::Items(vec![tag]));
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

exec!(CustomQuery);

impl <'g> Executor<'g> {

    pub fn execute<T>(self) -> Result<(Headers, StatusCode, Option<T>)>
        where T: DeserializeOwned {
        let mut rsp = self.client.execute(self.request?)?;
        let mut data = Vec::new();
        rsp.read_to_end(&mut data)
            .chain_err(|| ErrorKind::Msg("failed to read data from the response".to_string()))?;
        let json = if data.is_empty() {
            None
        } else {
            Some(serde_json::from_slice(&data)
                .chain_err(|| ErrorKind::Msg("failed to deserialize JSON response".to_string()))?)
        };
        Ok((rsp.headers().clone(), rsp.status(), json))
    }

}

// From derivations of Github to the given type using a certain
// request method
from!(
    @GetQueryBuilder
        => Method::Get
    @PutQueryBuilder
        => Method::Put
    @PostQueryBuilder
        => Method::Post
    @PatchQueryBuilder
        => Method::Patch
    @DeleteQueryBuilder
        => Method::Delete
);

// Custom Url based from impls
from!(
    @GetQueryBuilder
       => CustomQuery
    @PutQueryBuilder
       => CustomQuery
    @PostQueryBuilder
       => CustomQuery
    @PatchQueryBuilder
       => CustomQuery
    @DeleteQueryBuilder
       => CustomQuery
    @CustomQuery
        => Executor
);
