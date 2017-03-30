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
    pub fn put_empty(&mut self) -> PutQueryBuilder {
        self.into()
    }
    pub fn put<T>(&mut self, body: T) -> PutQueryBuilder
        where T: Serialize {
        let mut qb: PutQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
                let serialized = serde_json::to_vec(&body);
                match serialized {
                    Ok(json) => {
                        qbr.set_body(json);
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
    pub fn post<T>(&mut self, body: T) -> PostQueryBuilder
        where T: Serialize {
        let mut qb: PostQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
                let serialized = serde_json::to_vec(&body);
                match serialized {
                    Ok(json) => {
                        qbr.set_body(json);
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
    pub fn patch<T>(&mut self, body: T) -> PatchQueryBuilder
        where T: Serialize {
        let mut qb: PatchQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
                let serialized = serde_json::to_vec(&body);
                match serialized {
                    Ok(json) => {
                        qbr.set_body(json);
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
    pub fn delete<T>(&mut self, body: T) -> DeleteQueryBuilder
        where T: Serialize {
        let mut qb: DeleteQueryBuilder = self.into();
        match qb.request {
            Ok(mut qbr) => {
                let serialized = serde_json::to_vec(&body);
                match serialized {
                    Ok(json) => {
                        qbr.set_body(json);
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
    pub fn delete_empty(&mut self) -> DeleteQueryBuilder {
        self.into()
    }

}

impl<'a> GetQueryBuilder<'a> {
    func_client!(emojis, misc::get::Emojis<'a>);
    func_client!(rate_limit, misc::get::RateLimit<'a>);
    func_client!(user, users::get::User<'a>);
    func_client!(users, users::get::Users<'a>);
}

impl<'a> PutQueryBuilder<'a> {
    func_client!(user, users::put::User<'a>);
}

impl<'a> DeleteQueryBuilder<'a> {
    func_client!(user, users::delete::User<'a>);
}

impl<'a> PostQueryBuilder<'a> {
    func_client!(user, users::post::User<'a>);
}

impl<'a> PatchQueryBuilder<'a> {
    func_client!(user, users::patch::User<'a>);
}

impl<'a> Executor<'a> {

    pub fn execute(self) -> Result<(StatusCode, Option<Json>)> {
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
                    if chunks.is_empty() {
                        (status, None)
                    } else {
                        (status, Some(serde_json::from_slice(&chunks).unwrap()))
                    }
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
