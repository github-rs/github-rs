//! Access the Repos portion of the GitHub API
use tokio_core::reactor::Core;
use hyper::client::Request;
use hyper::status::StatusCode;
use hyper::Body;
use errors::*;
use util::url_join;
use client::{GetQueryBuilder, Executor};
use Json;

new_type!(Assignees);
new_type!(Repo);
new_type!(Repos);
new_type!(Owner);

from!(Assignees, Executor);
from!(GetQueryBuilder, Repos, "repos");
from!(Owner, Repo);
from!(Repo, Assignees, "assignees");
from!(Repo, Executor);
from!(Repos, Owner);

impl<'a> Repos<'a> {
    func!(owner, Owner, username_str);
}

impl<'a> Owner<'a> {
    func!(repo, Repo, repo_str);
}

impl<'a> Repo<'a> {
    func!(assignees, Assignees);
    exec!();
}

impl<'a> Assignees<'a> {
    exec!();
}
