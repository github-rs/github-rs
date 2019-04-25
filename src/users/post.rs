//! Access the Users portion of the GitHub API
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    User
    Emails
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
