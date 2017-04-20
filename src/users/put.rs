//! Access the Users portion of the GitHub API
imports!();
use client::{ PutQueryBuilder, Executor };

new_type!(
    User
    Following
    Username
);

from!(
    @PutQueryBuilder
        -> User = "user"
    @User
        -> Following = "following"
    @Following
        => Username
    @Username
        => Executor
);

impl_macro!(
    @User
        |=> following -> Following
        |
    @Following
        |
        |=> username -> Username = username_str
    @Username
        |
        |-> execute
);
