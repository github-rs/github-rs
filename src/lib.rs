//! Library to used to access the Github API with Rust
#![allow(dead_code)] // Until every starting struct gets used

#![deny(//missing_docs,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use] mod macros;

pub mod errors {
    error_chain!{
        foreign_links {
            Reqwest(::reqwest::Error)
                #[doc = "An error from the reqwest crate."];
        }
    }
}
pub mod client;
pub mod gists;
pub mod headers;
pub mod issues;
pub mod misc;
pub mod notifications;
pub mod orgs;
pub mod repos;
pub mod search;
pub mod teams;
pub mod users;
