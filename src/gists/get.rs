//! Access the Gists portion of the Github API
imports!();
use client::GetQueryBuilder;

new_type!(
    GistsGistId
);

from!(
    @GetQueryBuilder
        -> Gists = "gists"
    @Gists
        => GistsId
        => GistsGistId
        -> GistsPublic = "public"
        -> GistsStarred = "starred"
    @GistsId
        => GistsIdCommits
        => GistsIdSha
        -> GistsIdStar = "star"
        -> GistsIdForks = "forks"
    @GistsGistId
        -> GistsGistIdComments = "comments"
    @GistsGistIdComments
        => GistsGistIdCommentsId
);

impl_macro!(
    @Gists
        |=> public -> GistsPublic
        |=> starred -> GistsStarred
        |
        |=> id -> GistsId = id_str
        |=> gist_id -> GistsGistId = gist_id_str
    @GistsId
        |=> star -> GistsIdStar
        |=> forks -> GistsIdForks
        |
        |=> commits -> GistsIdCommits = commits_str
        |=> sha -> GistsIdSha = sha_str
    @GistsGistId
        |=> comments -> GistsGistIdComments
        |
    @GistsGistIdComments
        |
        |=> id -> GistsGistIdCommentsId = id_str
);

exec!(Gists);
exec!(GistsGistIdComments);
exec!(GistsGistIdCommentsId);
exec!(GistsId);
exec!(GistsIdCommits);
exec!(GistsIdForks);
exec!(GistsIdSha);
exec!(GistsIdStar);
exec!(GistsPublic);
exec!(GistsStarred);
