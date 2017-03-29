//! Access the Users portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use util::url_join;
use client::Executor;
use Json;

// Declaration of types representing the various items under users
new_type!(Emails);
new_type!(Followers);
new_type!(Following);
new_type!(FollowingUser);
new_type!(Keys);
new_type!(KeysId);
new_type!(Orgs);
new_type!(User);
new_type!(Users);
new_type!(UsersKeys);
new_type!(Username);
new_type!(Repos);
new_type!(Issues);
new_type!(Subscriptions);

// From implementations for conversion
from!(Emails, Executor);
from!(Followers, Executor);
from!(Following, FollowingUser);
from!(Following, Executor);
from!(FollowingUser, Executor);
from!(Issues, Executor);
from!(Keys, KeysId);
from!(Keys, Executor);
from!(KeysId, Executor);
from!(Orgs, Executor);
from!(Subscriptions, Executor);
from!(User, Emails, "emails");
from!(User, Followers, "followers");
from!(User, Following, "following");
from!(User, Keys, "keys");
from!(User, Executor);
from!(User, Issues, "issues");
from!(User, Orgs, "orgs");
from!(User, Subscriptions, "subscriptions");
from!(Users, Executor);
from!(Users, Username);
from!(UsersKeys, Executor);
from!(Username, Followers, "followers");
from!(Username, Following, "following");
from!(Username, UsersKeys, "keys");
from!(Username, Repos, "repos");
from!(Username, Executor);
from!(User, Repos, "repos");
from!(Repos, Executor);

// impls of each type
impl<'a> User<'a> {
    func!(emails, Emails);
    func!(followers, Followers);
    func!(following, Following);
    func!(issues, Issues);
    func!(repos, Repos);
    func!(subscriptions, Subscriptions);
    func!(keys, Keys);
    func!(orgs, Orgs);
    exec!();
}

impl<'a> Users<'a> {
    func!(username, Username, username_str);
    exec!();
}

impl<'a> Username<'a> {
    func!(followers, Followers);
    func!(following, Following);
    func!(keys, UsersKeys);
    func!(repos, Repos);
    exec!();
}

impl<'a> Keys<'a> {
    // This is a status based call, will need to figure out
    func!(id, KeysId, id_str);
    exec!();
}

impl<'a> Following<'a> {
    // This is a status based call, will need to figure out
    func!(username, Following, username_str);
    exec!();
}

exec!(UsersKeys);
exec!(Emails);
exec!(FollowingUser);
exec!(Issues);
exec!(KeysId);
exec!(Followers);
exec!(Repos);
exec!(Subscriptions);
exec!(Orgs);
