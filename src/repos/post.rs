//! Access the Repos portion of the GitHub API
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Sha
    Statuses
    Repo
    Repos
    Owner
    Issues
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
