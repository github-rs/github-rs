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
use serde_json;

// Internal Library Imports
use users::get::{User, Users};
use misc::get::*;
use util::url_join;
use errors::*;
use Json;

/// Struct used to make calls to the Github API.
pub struct Github {
    token: String,
    core: Core,
}
new_type!(GetQueryBuilder);
new_type!(PutQueryBuilder);
new_type!(PostQueryBuilder);
new_type!(DeleteQueryBuilder);
new_type!(PatchQueryBuilder);
new_type!(Executor);


impl Github {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
            core: Core::new().unwrap(),
        }
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_owned();
    }
    pub fn get(&mut self) -> GetQueryBuilder {
        self.into()
    }
    pub fn put(&mut self) -> PutQueryBuilder {
        self.into()
    }
    pub fn post(&mut self) -> PostQueryBuilder {
        self.into()
    }
    pub fn patch(&mut self) -> PatchQueryBuilder {
        self.into()
    }
    pub fn delete(&mut self) -> DeleteQueryBuilder {
        self.into()
    }

}

impl<'a> GetQueryBuilder<'a> {
    func!(rate_limit, RateLimit);
    func!(user, User);
    func!(users, Users);
}

impl<'a> Executor<'a> {

    pub fn execute(self) -> Result<(StatusCode, Json)> {
        let handle = self.core.handle();
        let work = Client::configure()
            .connector(HttpsConnector::new(4,&handle))
            .build(&handle)
            // All that type checking abuse culminates in this checking
            // if the request actually didn't fail being built!
            .request(self.request?)
            .and_then(|res| {
                let status = res.status().clone();
                res.body().fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    ok::<_, Error>(v)
                }).map(move |chunks| {
                    (status, serde_json::from_slice(&chunks).unwrap())
                })
            });

        self.core.run(work).chain_err(|| "Failed to execute request")
    }

}

// From derivations of Github to the given type using a certain
// request method
from!(GetQueryBuilder, Method::Get);
from!(PutQueryBuilder, Method::Put);
from!(PostQueryBuilder, Method::Post);
from!(PatchQueryBuilder, Method::Patch);
from!(DeleteQueryBuilder, Method::Delete);

from!(GetQueryBuilder, RateLimit, "rate_limit");
from!(GetQueryBuilder, User, "user");
from!(GetQueryBuilder, Users, "users");
