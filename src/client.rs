// Tokio/Future Imports
use futures::{ Future, Stream };
use futures::future::ok;
use tokio_core::reactor::Core;

// Hyper Imports
use hyper::{ Body, Uri, Method, Error };
use hyper::client::{ Client, Request };
use hyper::header::{ Authorization, Accept, ContentType, UserAgent, qitem };
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
}
new_type!(GetQueryBuilder);
new_type!(PutQueryBuilder);
new_type!(PostQueryBuilder);
new_type!(DeleteQueryBuilder);
new_type!(PatchQueryBuilder);
new_type!(CustomQuery);
new_type!(Executor);


impl Github {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
            core: Rc::new(RefCell::new(Core::new().unwrap())),
        }
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_owned();
    }
    pub fn get(self) -> GetQueryBuilder {
        self.into()
    }
    pub fn put_empty(self) -> PutQueryBuilder {
        self.into()
    }
    pub fn put<T>(self, body: T) -> PutQueryBuilder
        where T: Serialize {
        let mut qb: PutQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
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
            },
            Err(_) => {},
        }
        qb
    }
    pub fn post<T>(self, body: T) -> PostQueryBuilder
        where T: Serialize {
        let mut qb: PostQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
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
            },
            Err(_) => {},
        }

        qb
    }
    pub fn patch<T>(self, body: T) -> PatchQueryBuilder
        where T: Serialize {
        let mut qb: PatchQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
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
            },
            Err(_) => {},
        }
        qb
    }
    pub fn delete<T>(self, body: T) -> DeleteQueryBuilder
        where T: Serialize {
        let mut qb: DeleteQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
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
            },
            Err(_) => {},
        }
        qb
    }
    pub fn delete_empty(self) -> DeleteQueryBuilder {
        self.into()
    }

}

impl GetQueryBuilder {
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
    func!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(emojis, misc::get::Emojis);
    func_client!(rate_limit, misc::get::RateLimit);
    func_client!(user, users::get::User);
    func_client!(users, users::get::Users);
    func_client!(repos, repos::get::Repos);
}

impl PutQueryBuilder {
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
    func!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::put::User);
}

impl DeleteQueryBuilder {
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
    func!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::delete::User);
}

impl PostQueryBuilder {
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
    func!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::post::User);
}

impl PatchQueryBuilder {
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
    func!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(user, users::patch::User);
}

exec!(CustomQuery);

impl Executor {

    pub fn execute(self) -> Result<(StatusCode, Option<Json>)> {
        let ref mut core_ref = *self.core
                                    .try_borrow_mut()
                                    .chain_err(|| "Unable to get mutable borrow\
                                                   to the event loop")?;
        let handle = core_ref.handle();
        let work = Client::configure()
            .connector(HttpsConnector::new(4,&handle))
            .build(&handle)
            // All that type checking abuse culminates in this checking
            // if the request actually didn't fail being built!
            .request(self.request?.into_inner())
            .and_then(|res| {
                let status = res.status().clone();
                res.body().fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    ok::<_, Error>(v)
                }).map(move |chunks| {
                    if chunks.is_empty() {
                        (status, None)
                    } else {
                        (status, Some(serde_json::from_slice(&chunks).unwrap()))
                    }
                })
            });

        core_ref.run(work).chain_err(|| "Failed to execute request")
    }

}

// From derivations of Github to the given type using a certain
// request method
from!(GetQueryBuilder, Method::Get);
from!(PutQueryBuilder, Method::Put);
from!(PostQueryBuilder, Method::Post);
from!(PatchQueryBuilder, Method::Patch);
from!(DeleteQueryBuilder, Method::Delete);

// Custom Url based from impls
from!(GetQueryBuilder, CustomQuery);
from!(PutQueryBuilder, CustomQuery);
from!(PostQueryBuilder, CustomQuery);
from!(PatchQueryBuilder, CustomQuery);
from!(DeleteQueryBuilder, CustomQuery);
from!(CustomQuery, Executor);
