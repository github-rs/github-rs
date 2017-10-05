//! Access the Repos portion of the GitHub API
imports!();
use client::PostQueryBuilder;

new_type!(
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
        -> Issues = "issues"
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
        |=> issues -> Issues
        |
    @Statuses
        |
        |=> sha -> Sha = sha_str
    @Issues
        |
);

exec!(Sha);
exec!(Issues);
