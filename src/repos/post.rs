//! Access the Repos portion of the GitHub API
imports!();
use client::PostQueryBuilder;

new_type!(
    Sha
    Statuses
    Repo
    Repos
    Owner
);

from!(
    @PostQueryBuilder
        -> Repos = "repos"
    @Repos
        => Owner
    @Owner
        => Repo
    @Repo
        -> Statuses = "statuses"
    @Statuses
        => Sha
);

impl_macro!(
    @Repos
        |
        |=> owner ->  Owner = username_str
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> statuses -> Statuses
        |
    @Statuses
        |
        |=> sha -> Sha = sha_str
);

exec!(Sha);
