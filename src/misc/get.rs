//! Access the Misc portion of the GitHub API
imports!();
use client::GetQueryBuilder;

new_type!(
    Emojis
    Events
    Feeds
    Meta
    RateLimit
);

from!(
    @GetQueryBuilder
       -> Emojis = "emojis"
       -> Events = "events"
       -> Feeds = "feeds"
       -> Meta = "meta"
       -> RateLimit = "rate_limit"
);

exec!(Emojis);
exec!(Events);
exec!(Feeds);
exec!(Meta);
exec!(RateLimit);
