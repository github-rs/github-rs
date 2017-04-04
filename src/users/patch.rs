//! Access the Users portion of the GitHub API
imports!();
use client::{ PatchQueryBuilder, Executor };

new_type!(User);
new_type!(Email);
new_type!(Visibility);

from!(PatchQueryBuilder, User, "user");
from!(User, Email, "email");
from!(Email, Visibility, "visibility");
from!(Visibility, Executor);

impl <'g> User<'g> {
    func!(emails, Email);
}

impl <'g> Email<'g> {
    func!(visibility, Visibility);
}

exec!(Visibility);
