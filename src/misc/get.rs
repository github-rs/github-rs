//! Access the Misc portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

from!(GetQueryBuilder, Emojis, "emojis");
from!(GetQueryBuilder, Events, "events");
from!(GetQueryBuilder, Feeds, "feeds");
from!(GetQueryBuilder, Meta, "meta");
from!(GetQueryBuilder, RateLimit, "rate_limit");

new_type!(Emojis);
new_type!(Events);
new_type!(Feeds);
new_type!(Meta);
new_type!(RateLimit);

from!(Emojis, Executor);
from!(Events, Executor);
from!(Feeds, Executor);
from!(Meta, Executor);
from!(RateLimit, Executor);

exec!(Emojis);
exec!(Events);
exec!(Feeds);
exec!(Meta);
exec!(RateLimit);
