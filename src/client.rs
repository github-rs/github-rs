// Tokio/Future Imports
use futures::{ Future, Stream };
use futures::future::ok;
use tokio_core::reactor::Core;

// Hyper Imports
use hyper::{ Body, Headers, Uri, Method, Error };
use hyper::client::{ Client, Request };
use hyper::header::{ Authorization, Accept, ContentType,
                     ETag, IfNoneMatch, UserAgent, qitem };
use hyper::mime::Mime;
use hyper::status::StatusCode;
use hyper_tls::HttpsConnector;

// Serde Imports
use serde::Serialize;
use serde_json;

// Internal Library Imports
use users;
use misc;
use repos;
use errors::*;
use util::url_join;
use Json;

use std::rc::Rc;
use std::cell::RefCell;

/// Struct used to make calls to the Github API.
pub struct Github {
    token: String,
    core: Rc<RefCell<Core>>,
    client: Rc<Client<HttpsConnector>>,
}

impl Clone for Github {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            core: self.core.clone(),
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
    pub fn new<T>(token: T) -> Self
        where T: AsRef<str> {
        let core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4,&handle))
            .build(&handle);
        Self {
            token: token.as_ref().into(),
            core: Rc::new(RefCell::new(core)),
            client: Rc::new(client),
        }
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Change the currently set Authorization Token using a type that can turn
    /// into an &str. Must be a valid API Token for requests to work.
    pub fn set_token<T>(&mut self, token: T)
        where T: AsRef<str> {
        self.token = token.as_ref().into();
    }

    /// Exposes the inner event loop for those who need
    /// access to it. The reccomended way to safely access
    /// the core would be
    ///
    /// ```text
    /// let g = Github::new("API KEY");
    /// let core = g.get_core();
    /// // Handle the error here.
    /// let ref mut core_mut = *core.try_borrow_mut()?;
    /// // Do stuff with the core here. This prevents a runtime failure by
    /// // having two mutable borrows to the core at the same time.
    /// ```
    ///
    /// This is how other parts of the API are implemented to avoid causing your
    /// program to crash unexpectedly. While you could borrow without the
    /// `Result` being handled it's highly reccomended you don't unless you know
    /// there is no other mutable reference to it.
    pub fn get_core(&self) -> &Rc<RefCell<Core>> {
        &self.core
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
                    qbr.get_mut().set_body(json);
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
                    qbr.get_mut().set_body(json);
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
                    qbr.get_mut().set_body(json);
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
                    qbr.get_mut().set_body(json);
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

    /// Query the emojis endpoint
    func_client!(events, misc::get::Events<'g>);

    /// Query the emojis endpoint
    func_client!(feeds, misc::get::Feeds<'g>);

    /// Query the emojis endpoint
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
                req.get_mut().headers_mut().set(IfNoneMatch::Items(vec![tag]));
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
                req.get_mut().headers_mut().set(IfNoneMatch::Items(vec![tag]));
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
                req.get_mut().headers_mut().set(IfNoneMatch::Items(vec![tag]));
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

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: ETag) -> Self {
        match self.request {
            Ok(mut req) => {
                let ETag(tag) = tag;
                req.get_mut().headers_mut().set(IfNoneMatch::Items(vec![tag]));
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
                req.get_mut().headers_mut().set(IfNoneMatch::Items(vec![tag]));
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

exec!(CustomQuery);

impl <'g> Executor<'g> {

    pub fn execute(self) -> Result<(Headers, StatusCode, Option<Json>)> {
        let mut core_ref = self.core
                            .try_borrow_mut()
                            .chain_err(|| "Unable to get mutable borrow\
                                            to the event loop")?;
        let client = self.client;
        let work = client
                    .request(self.request?.into_inner())
                    .and_then(|res| {
                        let header = res.headers().clone();
                        let status = res.status();
                        res.body().fold(Vec::new(), |mut v, chunk| {
                            v.extend(&chunk[..]);
                            ok::<_, Error>(v)
                        }).map(move |chunks| {
                            if chunks.is_empty() {
                                (header, status, None)
                            } else {
                                (
                                  header,
                                  status,
                                  Some(serde_json::from_slice(&chunks).unwrap())
                                )
                            }
                        })
                    });
        core_ref.run(work).chain_err(|| "Failed to execute request")
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
