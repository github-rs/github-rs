//! Access the Users portion of the GitHub API
imports!();
use crate::client::PutQueryBuilder;

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
