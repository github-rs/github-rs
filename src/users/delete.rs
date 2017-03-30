//! Access the Users portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use util::url_join;
use client::{ DeleteQueryBuilder, Executor };
use Json;

new_type!(User);
new_type!(Emails);

from!(DeleteQueryBuilder, User, "user");
from!(User, Emails, "emails");
from!(Emails, Executor);

impl<'a> User<'a> {
    func!(emails, Emails);
}

exec!(Emails);
