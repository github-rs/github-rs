// Tokio/Future Imports
use futures::future::ok;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

use serde_json;
// Hyper Imports
use hyper::client::Client;
use hyper::StatusCode;
use hyper::{self, Headers};
#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnector;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

// Serde Imports
use serde::de::DeserializeOwned;

// Lib Imports
use errors::*;
use mutation::Mutation;
use query::Query;
use IntoGithubRequest;

// Std Imports
use std::cell::RefCell;
use std::rc::Rc;

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

impl Github {
    /// Create a new Github client struct. It takes a type that can convert into
    /// a `String` (`&str` or `Vec<u8>` for example). As long as the function is
    /// given a valid API Token your requests will work.
    pub fn new<T>(token: T) -> Result<Self>
    where
        T: ToString,
    {
        let core = Core::new()?;
        let handle = core.handle();
        #[cfg(feature = "rustls")]
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle))
            .build(&handle);
        #[cfg(feature = "rust-native-tls")]
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle)?)
            .build(&handle);
        Ok(Self {
            token: token.to_string(),
            core: Rc::new(RefCell::new(core)),
            client: Rc::new(client),
        })
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Change the currently set Authorization Token using a type that can turn
    /// into an &str. Must be a valid API Token for requests to work.
    pub fn set_token<T>(&mut self, token: T)
    where
        T: ToString,
    {
        self.token = token.to_string();
    }

    /// Exposes the inner event loop for those who need
    /// access to it. The recommended way to safely access
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
    /// `Result` being handled it's highly recommended you don't unless you know
    /// there is no other mutable reference to it.
    pub fn get_core(&self) -> &Rc<RefCell<Core>> {
        &self.core
    }

    pub fn query<T>(&mut self, query: &Query) -> Result<(Headers, StatusCode, Option<T>)>
    where
        T: DeserializeOwned,
    {
        self.run(query)
    }

    pub fn mutation<T>(&mut self, mutation: &Mutation) -> Result<(Headers, StatusCode, Option<T>)>
    where
        T: DeserializeOwned,
    {
        self.run(mutation)
    }

    fn run<T, I>(&mut self, request: &I) -> Result<(Headers, StatusCode, Option<T>)>
    where
        T: DeserializeOwned,
        I: IntoGithubRequest,
    {
        let mut core_ref = self.core.try_borrow_mut().chain_err(|| {
            "Unable to get mutable borrow \
             to the event loop"
        })?;
        let client = &self.client;
        let work = client
            .request(request.into_github_req(&self.token)?)
            .and_then(|res| {
                let header = res.headers().clone();
                let status = res.status();
                res.body()
                    .fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        ok::<_, hyper::Error>(v)
                    })
                    .map(move |chunks| {
                        if chunks.is_empty() {
                            Ok((header, status, None))
                        } else {
                            Ok((
                                header,
                                status,
                                Some(
                                    serde_json::from_slice(&chunks)
                                        .chain_err(|| "Failed to parse response body")?,
                                ),
                            ))
                        }
                    })
            });
        core_ref
            .run(work)
            .chain_err(|| "Failed to execute request")?
    }
}
