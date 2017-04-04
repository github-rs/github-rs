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

impl <'g> User<'g> {
    func!(following, Following);
}

impl <'g> Following<'g> {
    func!(username, Username, username_str);
}

exec!(Username);
