// We only are using thses crates to transform Hyper errors
use solicit::http::HttpError;
use url::ParseError;

// Imports from crates used elsewhere and the standard library
use serde_json::error as serdeErr;
use hyper::error as hyperErr;
use std::error;
use std::io;
use std::fmt;
use std::result;
use std::string;
use std::str::Utf8Error;
// -----------------------------------------------------------------------------------------//
//                              ERROR TYPE ALIASES                                         //
// -----------------------------------------------------------------------------------------//

/// Result type used throughout all the endpoints to unify them all under one error type and to
/// make the signature heading easier to read
pub type Result<T> = result::Result<T, GithubError>;
/// Used to make it more clear that the error has to do with an underlying Http2 library and is not
/// a problem with http itself
pub type Http2Error = HttpError;

// -----------------------------------------------------------------------------------------//
//                             GITHUB ERROR IMPLEMENTATION                                 //
// -----------------------------------------------------------------------------------------//
/// Errors related to improper usage of the API
#[derive(Debug)]
pub enum GithubError {
    // SERDE ERRORS
    /// Encountered if serde cannot deserialize JSON returned from the API or it cannot serialize
    /// a struct into JSON to be sent to the API.
    JsonParsingSyntax(serdeErr::ErrorCode, usize, usize),
    /// While parsing with serde an IO error occured.
    JsonParsingIo(io::Error),
    /// While converting from Utf8 into a byte string for the request to the API an error occured
    JsonParsingFromUtf8(string::FromUtf8Error),
    // HYPER ERRORS
    /// A General error for the hyper library when none of the other errors work for it.
    Request,
    /// While setting up the method for the request an error occurred (i.e. `PST` instead of
    /// `POST`)
    RequestMethod,
    /// While parsing the Uri for the request an error occurred
    RequestUri(ParseError),
    /// An error occured with the version used to communicate (i.e. `HTP/1.1` instead of
    /// `HTTP/1.1`)
    RequestVersion,
    /// An error in creating the header occurred  making the request invalid
    RequestHeader,
    /// The request being sent is far to large to work
    RequestTooLarge,
    /// An invalid status code was retuned
    RequestStatus,
    /// While performing an IO action with hyper an error occurred
    RequestIo(io::Error),
    /// While intitiating, in the process of, or while terminating an SSL connection with the API
    /// an error occurred.
    RequestSsl(Box<error::Error + Send + Sync>),
    /// While attemptin to make a connection via Http2 an error occured
    RequestHttp2(Http2Error),
    /// An error within the byte string for the request to the API occured
    RequestUtf8(Utf8Error),
    /// Encountered if whatever was returned was not at all JSON
    NonJsonBody,
    /// Encountered if you've made a request with improper JSON. Most likely encountered
    /// where serde serializes a struct incorrectly, otherwise type checking prevents this error.
    InvalidFields,
    /// You aren't authorized to make the request. Make sure your token's scope allows you to make
    /// the call to the API that you would expect
    Unauthorized,
    /// Github puts a rate limit on API calls (you get more if you're authenticated). If that
    /// happens you'll get this error.
    QueryLimit,
    /// While performing an IO action within the github-rs library an error occured.
    LibIo(io::Error),
    /// The Github API returned a 404 object.
    Github404,
    /// The github-rs library encountered an error making a Mime type for the header.
    Mime,
    /// The catch all error. If it's not one of the above then something in the implementation of
    /// github-rs went absolutely wrong. A bug report is greatly appreciated if you encounter this error.
    LibError,
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use self::GithubError::*;
        match *self {
            // JsonParsing(ref err) => write!(f, "{}", err),
            _ => write!(f, "{}", self),
        }
    }
}

impl error::Error for GithubError {
    fn description(&self) -> &str {
        use self::GithubError::*;
        match *self {
            JsonParsingSyntax(..) => "Syntax Error",
            JsonParsingIo(ref err) | RequestIo(ref err) => error::Error::description(err),
            JsonParsingFromUtf8(..) => "UTF-8 Error",
            RequestMethod => "Method in Headers was malformed",
            RequestUri(ref err) => error::Error::description(err),
            RequestVersion => "Request protocol in Headers was malformed",
            RequestHeader => "Headers were malformed",
            RequestTooLarge => "Request payload is too big to work",
            RequestStatus => "Status Code returned was not a valid code",
            RequestSsl(..) => "While performing SSL actions with the request an error occurred",
            RequestHttp2(ref err) => error::Error::description(err),
            RequestUtf8(ref err) => error::Error::description(err),
            Request => "Something went wrong while connecting. We are unsure what the problem is.",
            // Github-rs Specific Errors not due to other libraries
            LibIo(..) => "Error occured while performing internal library Io Actions",
            Github404 => "Message: Not Found, Documentation URL: https://developer.github.com/v3",
            NonJsonBody => "JSON was not returned",
            InvalidFields => "You've made a request with invalid fields",
            Unauthorized => "Your request was unauthorized. Check your OAuth Token.",
            QueryLimit => "You've hit your query limit to the API.",
            Mime => "While making a Mime type for the Header an error occurred",
            LibError => {
                "The github-rs lib has caused an error. Please file a bug report with your \
                 request, what you expected, and what method you used that failed."
            }
        }
    }
}


// -----------------------------------------------------------------------------------------//
//                                      FROM IMPLS                                         //
// -----------------------------------------------------------------------------------------//

impl From<serdeErr::Error> for GithubError {
    fn from(serde: serdeErr::Error) -> Self {
        use self::GithubError::*;
        use serde_json::error::Error::*;
        match serde {
            Syntax(x, y, z) => JsonParsingSyntax(x, y, z),
            Io(err) => JsonParsingIo(err),
        }
    }
}

impl From<hyperErr::Error> for GithubError {
    fn from(hyper: hyperErr::Error) -> Self {
        use self::GithubError::*;
        use hyper::error::Error::*;
        match hyper {
            Method => RequestMethod,
            Uri(err) => RequestUri(err),
            Version => RequestVersion,
            Header => RequestHeader,
            TooLarge => RequestTooLarge,
            Status => RequestStatus,
            Io(err) => RequestIo(err),
            Ssl(err) => RequestSsl(err),
            Http2(err) => RequestHttp2(err),
            Utf8(err) => RequestUtf8(err),
            _ => Request,
        }
    }
}
// -----------------------------------------------------------------------------------------//
//                                          MACROS                                         //
// -----------------------------------------------------------------------------------------//

// Unwrapping macro for serde
macro_rules! try_serde {
    ($x:expr) => (
        match $x {
            Ok(x) => Ok(x),
            // Like try we want to return on error immediately
            Err(err) => Err(GithubError::from(err)),
        }
    );
}

// Unwrapping macro for hyper
// It acts like try but unlike our serde macro it's used for the internal library. This just
// unwraps the value like try but converts to a GithubError otherwise
macro_rules! try_hyper {
    ($x:expr) => (
        match $x {
            Ok(x) => x,
            // Like try we want to return on error immediately
            Err(err) => return Err(GithubError::from(err)),
        }
    );
}

// Unwrapping macro for Mime types
// It acts like try but unlike our serde or hyper macro it's used for the internal library. This just
// unwraps the value like try but converts to a GithubError otherwise
macro_rules! try_mime {
    ($x:expr) => (
        match $x {
            Ok(x) => x,
            // Like try we want to return on error immediately
            Err(_) => return Err(GithubError::Mime),
        }
    );
}
