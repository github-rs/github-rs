//! Access the Teams portion of the Github API
imports!();
use client::PatchQueryBuilder;

new_type!(
    Teams
    TeamsId
);

from!(
    @PatchQueryBuilder
        -> Teams = "teams"
    @Teams
        => TeamsId
);

impl_macro!(
    @Teams
        |
        |=> id -> TeamsId = id_str
);

exec!(TeamsId);
