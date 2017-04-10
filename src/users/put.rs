//! Access the Users portion of the GitHub API
imports!();
use client::{ PutQueryBuilder, Executor };

new_type!(User);
new_type!(Following);
new_type!(Username);

from!(PutQueryBuilder, User, "user");
from!(User, Following, "following");
from!(Following, Username);
from!(Username, Executor);

impl User {
    func!(following, Following);
}

impl Following {
    func!(username, Username, username_str);
}

exec!(Username);
