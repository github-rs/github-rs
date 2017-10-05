//! Access the Users portion of the GitHub API
imports!();
use client::PostQueryBuilder;

new_type!(
    User
);

from!(
    @PostQueryBuilder
        -> User = "user"
    @User
        -> Emails = "emails"
);

impl_macro!(
    @User
        |=> emails -> Emails
        |
);

exec!(Emails);
