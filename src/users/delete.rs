//! Access the Users portion of the GitHub API
imports!();
use client::DeleteQueryBuilder;

new_type!(
    User
    Emails
);

from!(
    @DeleteQueryBuilder
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
