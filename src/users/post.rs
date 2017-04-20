//! Access the Users portion of the GitHub API
imports!();
use client::{PostQueryBuilder, Executor};

new_type!(
    User
    Emails
);

from!(
    @PostQueryBuilder
        -> User = "user"
    @User
        -> Emails = "emails"
    @Emails
        => Executor
);

impl_macro!(
    @User
        |=> emails -> Emails
        |
    @Emails
        |
        |-> execute
);
