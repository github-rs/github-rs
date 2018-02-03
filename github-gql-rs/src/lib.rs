//! Library to used to access the Github API with Rust

#[macro_use]
extern crate error_chain;
extern crate hyper;
#[cfg(feature = "rustls")]
extern crate hyper_rustls;
#[cfg(feature = "rust-native-tls")]
extern crate hyper_tls;
#[cfg(feature = "rust-native-tls")]
extern crate native_tls;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

pub mod errors {
    error_chain!{
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
pub mod query;
pub mod mutation;

pub use hyper::{Headers, StatusCode};

use errors::Result;
pub trait IntoGithubRequest {
    fn into_github_req(&self, token: &str) -> Result<hyper::Request>;
}
