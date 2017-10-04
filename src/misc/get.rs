//! Access the Misc portion of the GitHub API
imports!();
use client::GetQueryBuilder;

new_type!(
    Emojis
    Events
    Feeds
    Gitignore
    GitignoreTemplates
    GitignoreTemplatesLang
    Meta
    RateLimit
);

from!(
    @GetQueryBuilder
       -> Emojis = "emojis"
       -> Events = "events"
       -> Feeds = "feeds"
       -> Gitignore = "gitignore"
       -> Meta = "meta"
       -> RateLimit = "rate_limit"
    @Gitignore
       -> GitignoreTemplates = "templates"
    @GitignoreTemplates
       => GitignoreTemplatesLang
);

impl_macro!(
    @Gitignore
       |=> templates -> GitignoreTemplates
       |
    @GitignoreTemplates
       |
       |=> lang -> GitignoreTemplatesLang = lang_str
);

exec!(Emojis);
exec!(Events);
exec!(Feeds);
exec!(GitignoreTemplates);
exec!(GitignoreTemplatesLang);
exec!(Meta);
exec!(RateLimit);
