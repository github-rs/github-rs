//! Access the Gists portion of the Github API
imports!();
use client::PostQueryBuilder;

new_type!(
    GistsId
    GistsGistId
);

from!(
    @PostQueryBuilder
        -> Gists = "gists"
    @Gists
        => GistsId
        => GistsGistId
    @GistsId
        -> GistsIdForks = "forks"
    @GistsGistId
        -> GistsGistIdComments = "comments"
);

impl_macro!(
    @Gists
        |
        |=> id -> GistsId = id_str
        |=> gist_id -> GistsGistId = gist_id_str
    @GistsId
        |=> forks -> GistsIdForks
        |
    @GistsGistId
        |=> gist_id -> GistsGistIdComments
        |
);

exec!(Gists);
exec!(GistsIdForks);
exec!(GistsGistIdComments);
