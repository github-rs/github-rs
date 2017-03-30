//! Access the Misc portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use client::{GetQueryBuilder, Executor};
use util::url_join;
use Json;

from!(GetQueryBuilder, Emojis, "emojis");
from!(GetQueryBuilder, RateLimit, "rate_limit");
new_type!(Emojis);
new_type!(RateLimit);

from!(Emojis, Executor);
from!(RateLimit, Executor);

exec!(Emojis);
exec!(RateLimit);
