//! Access the Users portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use util::url_join;
use client::{ PutQueryBuilder, Executor };
use Json;

new_type!(User);
new_type!(Following);
new_type!(Username);

from!(PutQueryBuilder, User, "user");
from!(User, Following, "following");
from!(Following, Username);
from!(Username, Executor);

impl<'a> User<'a> {
    func!(following, Following);
}

impl<'a> Following<'a> {
    func!(username, Username, username_str);
}

exec!(Username);
