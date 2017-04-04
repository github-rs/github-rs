//! Access the Users portion of the GitHub API
imports!();
use client::{PostQueryBuilder, Executor};

new_type!(User);
new_type!(Emails);

from!(PostQueryBuilder, User, "user");
from!(User, Emails, "emails");
from!(Emails, Executor);

impl <'g> User<'g> {
    func!(emails, Emails);
}

exec!(Emails);
