//! Access the Repos portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Assignees
    Branches
    Collaborators
    Repo
    Repos
    Owner
);

from!(
    @Assignees
       => Executor
    @Branches
       => Executor
    @Collaborators
       => Executor
    @GetQueryBuilder
       -> Repos = "repos"
    @Owner
       => Repo
    @Repo
       -> Assignees = "assignees"
    @Repo
       -> Branches = "branches"
    @Repo
       -> Collaborators = "collaborators"
    @Repo
       => Executor
    @Repos
       => Owner
);

impl_macro!(
    @Assignees
        |
        |-> execute
    @Branches
        |
        |-> execute
    @Collaborators
        |
        |-> execute

    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> assignees ->  Assignees
        |=> branches ->  Branches
        |=> collaborators ->  Collaborators
        |
        |-> execute

    @Repos
        |
        |=> owner ->  Owner = username_str
);
