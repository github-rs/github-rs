//! Access the Gists portion of the Github API
imports!();
use crate::client::DeleteQueryBuilder;

new_type!(
    Gists
    GistsId
    GistsIdStar
    GistsGistId
    GistsGistIdComments
    GistsGistIdCommentsId
);

from!(
    @DeleteQueryBuilder
        -> Gists = "gists"
    @Gists
        => GistsId
        => GistsGistId
    @GistsId
        -> GistsIdStar = "star"
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
    @GistsId
        |=> star -> GistsIdStar
        |
    @GistsGistId
        |=> comments -> GistsGistIdComments
        |
    @GistsGistIdComments
        |
        |=> id -> GistsGistIdCommentsId = id_str
);

exec!(GistsId);
exec!(GistsIdStar);
exec!(GistsGistIdCommentsId);
