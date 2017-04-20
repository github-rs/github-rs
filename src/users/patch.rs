//! Access the Users portion of the GitHub API
imports!();
use client::{ PatchQueryBuilder, Executor };

new_type!(
    User
    Email
    Visibility
);

from!(
    @PatchQueryBuilder
        -> User = "user"
    @User
        -> Email = "email"
    @Email
        -> Visibility = "visibility"
    @Visibility
        => Executor
);

impl_macro!(
    @User
        |=> emails -> Email
        |
    @Email
        |=> visibility -> Visibility
        |
    @Visibility
        |
        |-> execute
);
