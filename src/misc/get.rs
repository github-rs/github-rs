//! Access the Misc portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

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
    @Emojis
       => Executor
    @Events
       => Executor
    @Feeds
       => Executor
    @Meta
       => Executor
    @RateLimit
       => Executor
);

impl_macro!(
    @Emojis
        |
        |-> execute
    @Events
        |
        |-> execute
    @Feeds
        |
        |-> execute
    @Meta
        |
        |-> execute
    @RateLimit
        |
        |-> execute
);
