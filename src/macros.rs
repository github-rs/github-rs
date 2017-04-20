/// Automatically generate From impls for types given using a small DSL like
/// macrot
macro_rules! from {
    ($(@$f: ident $( => $t: ident )* $( -> $i: ident = $e: expr )*)*) => (
        $($(
        impl <'g> From<$f<'g>> for $t<'g> {
            fn from(f: $f<'g>) -> Self {
                Self {
                    request: f.request,
                    core: f.core,
                    client: f.client,
                }
            }
        }
        )*$(
        impl <'g> From<$f<'g>> for $i<'g> {
            fn from(mut f: $f<'g>) -> Self {
                // This is borrow checking abuse and about the only
                // time I'd do is_ok(). Essentially this allows us
                // to either pass the error message along or update
                // the url
                if f.request.is_ok() {
                    // We've checked that this works
                    let mut req = f.request.unwrap();
                    let url = url_join(req.get_mut().uri(), $e)
                        .chain_err(|| "Failed to parse Url");
                    match url {
                        Ok(u) => {
                            req.get_mut().set_uri(u);
                            f.request = Ok(req);
                        },
                        Err(e) => {
                            f.request = Err(e);
                        }
                    }

                    Self {
                        request: f.request,
                        core: f.core,
                        client: f.client,
                    }

                } else {

                    Self {
                        request: f.request,
                        core: f.core,
                        client: f.client,
                    }

                }
            }
        }
        )*)*
    );
    ($(@$t: ident => $p: path)*) => (
        $(
        impl <'g> From<&'g Github> for $t<'g> {
            fn from(gh: &'g Github) -> Self {
                use std::result;
                use errors;
                let url = "https://api.github.com".parse::<Uri>()
                    .chain_err(||
                        "Url failed to parse"
                    );
                let mime: result::Result<Mime, ()> =
                    "application/vnd.github.v3+json".parse();
                match (url, mime) {
                    (Ok(u), Ok(m)) => {
                        let mut req = Request::new($p, u);
                        let token = String::from("token ") + &gh.token;
                        {
                            let mut headers = req.headers_mut();
                            headers.set(ContentType::json());
                            headers.set(UserAgent::new(String::from("github-rs")));
                            headers.set(Accept(vec![qitem(m)]));
                            headers.set(Authorization(token));
                        }
                        Self {
                            request: Ok(RefCell::new(req)),
                            core: &gh.core,
                            client: &gh.client,
                        }
                    }
                    (Err(u), Ok(_)) => {
                        Self {
                            request: Err(u),
                            core: &gh.core,
                            client: &gh.client,
                        }
                    }
                    (Ok(_), Err(_)) => {
                        Self {
                            // Forgive me father for I have sinned and
                            // abused the error handling
                            request: Err(errors::Error::from_kind(
                                ErrorKind::from(
                                    "Mime failed to parse.".to_owned()
                                ))),
                            core: &gh.core,
                            client: &gh.client,
                        }
                    }
                    (Err(u), Err(_)) => {
                        Self {
                            request: Err(u).chain_err(||
                                "Mime failed to parse."
                            ),
                            core: &gh.core,
                            client: &gh.client,
                        }
                    }
                }
            }
        }
    )*
    );
}

/// Used to identify a new type used in a query pipeline. The types are
/// consistent between each one in terms of transforming one to another.
/// This helps reduce boiler plate code and makes it easy to expand and
/// maintain code in the future by simply adding a new field here if needed
macro_rules! new_type {
    ($($i: ident)*) => (
        $(
        pub struct $i<'g> {
            pub(crate) request: Result<RefCell<Request<Body>>>,
            pub(crate) core: &'g Rc<RefCell<Core>>,
            pub(crate) client: &'g Rc<Client<HttpsConnector>>,
        }
        )*
    );
}

/// Used to generate an execute function for a terminal type in a query
/// pipeline. If passed a type it creates the impl as well as it needs
/// no extra functions. If blank it just creates the function and should be
/// used this way only inside an impl
macro_rules! exec {
    () => (
        /// Execute the query by sending the built up request
        /// to GitHub. The value returned is either an error
        /// or the Status Code and Json after it has been deserialized.
        /// Please take a look at the GitHub documenation to see what value
        /// you should receive back for good or bad requests.
        pub fn execute(self) -> Result<(Headers, StatusCode, Option<Json>)> {
            let ex: Executor = self.into();
            ex.execute()
        }
    );
    ($t: ident) => (
        impl<'g> $t<'g> {
            /// Execute the query by sending the built up request
            /// to GitHub. The value returned is either an error
            /// or the Status Code and Json after it has been deserialized.
            /// Please take a look at the GitHub documenation to see what value
            /// you should receive back for good or bad requests.
            pub fn execute(self) ->
                Result<(Headers, StatusCode, Option<Json>)> {

                let ex: Executor = self.into();
                ex.execute()

            }
        }
    );
}

/// Using a small DSL like macro generate an impl for a given type
/// that creates all the functions to transition from one node type to another
macro_rules! impl_macro {
    ($(@$i: ident $(|=> $id1: ident -> $t1: ident)*|
     $(|=> $id2: ident -> $t2: ident = $e: ident)*
     $(|-> $id3: ident )*)+)=> (
        $(
            impl<'g> $i <'g>{
            $(
                pub fn $id1(self) -> $t1<'g> {
                    self.into()
                }
            )*$(
                pub fn $id2(mut self, $e: &str) -> $t2<'g> {
                    // This is borrow checking abuse and about the only
                    // time I'd do is_ok(). Essentially this allows us
                    // to either pass the error message along or update
                    // the url
                    if self.request.is_ok() {
                        // We've checked that this works
                        let mut req = self.request.unwrap();
                        let url = url_join(req.get_mut().uri(), $e)
                            .chain_err(|| "Failed to parse Url");
                        match url {
                            Ok(u) => {
                                req.get_mut().set_uri(u);
                                self.request = Ok(req);
                            },
                            Err(e) => {
                                self.request = Err(e);
                            }
                        }
                    }
                    self.into()
                }
            )*$(
                /// Execute the query by sending the built up request to GitHub.
                /// The value returned is either an error or the Status Code and
                /// Json after it has been deserialized. Please take a look at
                /// the GitHub documenation to see what value you should receive
                /// back for good or bad requests.
                pub fn $id3(self) -> Result<(Headers, StatusCode, Option<Json>)>
                {
                    let ex: Executor = self.into();
                    ex.execute()
                }
            )*
            }
        )+
    );
}

/// A variation of impl_macro for the client module that allows partitioning of
/// types. Create a function with a given name and return type. Used for
/// creating functions for simple conversions from one type to another, where
/// the actual conversion code is in the From implementation.
macro_rules! func_client{
    ($i: ident, $t: ty) => (
        pub fn $i(self) -> $t {
            self.into()
        }
    );
    ($i: ident, $t: ident, $e: ident) => (
        pub fn $i(mut self, $e: &str) -> $t<'g> {
            // This is borrow checking abuse and about the only
            // time I'd do is_ok(). Essentially this allows us
            // to either pass the error message along or update
            // the url
            if self.request.is_ok() {
                // We've checked that this works
                let mut req = self.request.unwrap();
                let url = url_join(req.get_mut().uri(), $e)
                    .chain_err(|| "Failed to parse Url");
                match url {
                    Ok(u) => {
                        req.get_mut().set_uri(u);
                        self.request = Ok(req);
                    },
                    Err(e) => {
                        self.request = Err(e);
                    }
                }
            }
            self.into()
        }
    );
}

/// Common imports for every file
macro_rules! imports{
    () => (
        use tokio_core::reactor::Core;
        use hyper_tls::HttpsConnector;
        use hyper::client::Client;
        use hyper::client::Request;
        use hyper::status::StatusCode;
        use hyper::{ Body, Headers };
        use errors::*;
        use util::url_join;
        use Json;
        use std::rc::Rc;
        use std::cell::RefCell;
    );
}
