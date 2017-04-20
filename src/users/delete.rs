//! Access the Users portion of the GitHub API
imports!();
use client::{ DeleteQueryBuilder, Executor };

new_type!(
    User
    Emails
);

from!(
    @DeleteQueryBuilder
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
