//! Access the Misc portion of the GitHub API
imports!();
use client::GetQueryBuilder;

new_type!(
    Gitignore
);

from!(
    @GetQueryBuilder
       -> Emojis = "emojis"
       -> Events = "events"
       -> Feeds = "feeds"
       -> Gitignore = "gitignore"
       -> Meta = "meta"
       -> RateLimit = "rate_limit"
       -> Organizations = "organizations"
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
exec!(Organizations);
