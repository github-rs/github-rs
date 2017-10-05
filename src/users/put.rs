//! Access the Users portion of the GitHub API
imports!();
use client::PutQueryBuilder;

new_type!(
    User
    Following
);

from!(
    @PutQueryBuilder
        -> User = "user"
    @User
        -> Following = "following"
    @Following
        => Username
);

impl_macro!(
    @User
        |=> following -> Following
        |
    @Following
        |
        |=> username -> Username = username_str
);

exec!(Username);
