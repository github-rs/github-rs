//! Library to used to access the Github API with Rust

#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
#[cfg(feature = "rustls")]
extern crate hyper_rustls;
#[cfg(feature = "rust-native-tls")]
extern crate hyper_tls;
#[cfg(feature = "rust-native-tls")]
extern crate native_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

#[allow(deprecated)] // error_chain uses cause()
pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error)
                #[doc = "`std::io::Error` converted to an error-chain type"];
            Serde(::serde_json::Error)
                #[doc = "`serde_json::Error` converted to an error-chain type"];
            Hyper(::hyper::Error)
                #[doc = "`hyper::Error` converted to an error-chain type"];
            NativeTls(::native_tls::Error)
                #[cfg(feature = "rust-native-tls")]
                #[doc = "`native_tls::Error` converted to an error-chain type"];
        }
    }
}

pub mod client;
pub mod mutation;
pub mod query;

pub use hyper::{HeaderMap, StatusCode};

use errors::Result;
pub trait IntoGithubRequest {
    fn into_github_req(&self, token: &str) -> Result<hyper::Request<hyper::Body>>;
}
