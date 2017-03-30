//! Access the Users portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use util::url_join;
use client::{ PatchQueryBuilder, Executor };
use Json;

new_type!(User);
new_type!(Email);
new_type!(Visibility);

from!(PatchQueryBuilder, User, "user");
from!(User, Email, "email");
from!(Email, Visibility, "visibility");
from!(Visibility, Executor);

impl<'a> User<'a> {
    func!(emails, Email);
}

impl<'a> Email<'a> {
    func!(visibility, Visibility);
}

exec!(Visibility);
