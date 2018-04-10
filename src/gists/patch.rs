//! Access the Gists portion of the GitHub API
imports!();
use client::PatchQueryBuilder;

new_type!(
    Gists
    GistsGistId
    GistsGistIdComments
);

from!(
    @PatchQueryBuilder
        -> Gists = "gists"
    @Gists
        => GistsId
        => GistsGistId
    @GistsGistId
        -> GistsGistIdComments = "comments"
    @GistsGistIdComments
        => GistsGistIdCommentsId
);

impl_macro!(
    @Gists
        |
        |=> id -> GistsId = id_str
        |=> gist_id -> GistsGistId = gist_id_str
    @GistsGistId
        |=> comments -> GistsGistIdComments
        |
    @GistsGistIdComments
        |
        |=> id -> GistsGistIdCommentsId = id_str
);

exec!(GistsId);
exec!(GistsGistIdCommentsId);
