//! Access the Users portion of the GitHub API
imports!();
use client::{ DeleteQueryBuilder, Executor };

new_type!(User);
new_type!(Emails);

from!(DeleteQueryBuilder, User, "user");
from!(User, Emails, "emails");
from!(Emails, Executor);

impl User {
    func!(emails, Emails);
}

exec!(Emails);
