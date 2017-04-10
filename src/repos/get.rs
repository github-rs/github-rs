//! Access the Repos portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

new_type!(Assignees);
new_type!(Branches);
new_type!(Collaborators);
new_type!(Repo);
new_type!(Repos);
new_type!(Owner);

from!(Assignees, Executor);
from!(Branches, Executor);
from!(Collaborators, Executor);
from!(GetQueryBuilder, Repos, "repos");
from!(Owner, Repo);
from!(Repo, Assignees, "assignees");
from!(Repo, Branches, "branches");
from!(Repo, Collaborators, "collaborators");
from!(Repo, Executor);
from!(Repos, Owner);


impl Assignees {
    exec!();
}

impl Branches {
    exec!();
}

impl Collaborators {
    exec!();
}

impl Owner {
    func!(repo, Repo, repo_str);
}

impl Repo {
    func!(assignees, Assignees);
    func!(branches, Branches);
    func!(collaborators, Collaborators);
    exec!();
}

impl Repos {
    func!(owner, Owner, username_str);
}
