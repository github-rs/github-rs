//! Access the Teams portion of the Github API
imports!();
use client::PutQueryBuilder;

new_type!(
    Teams
    TeamsId
    TeamsIdMemberships
    TeamsIdRepos
    TeamsIdReposOrg
    TeamsIdReposOrgRepo
    TeamsIdMembershipsUsername
);

from!(
    @PutQueryBuilder
        -> Teams = "teams"
    @Teams
        => TeamsId
    @TeamsId
        -> TeamsIdMemberships = "memberships"
        -> TeamsIdRepos = "repos"
    @TeamsIdRepos
        => TeamsIdReposOrg
    @TeamsIdReposOrg
        => TeamsIdReposOrgRepo
    @TeamsIdMemberships
        => TeamsIdMembershipsUsername
);

impl_macro!(
    @Teams
        |
        |=> id -> TeamsId = id_str
    @TeamsId
        |=> memberships -> TeamsIdMemberships
        |=> repos -> TeamsIdRepos
        |
    @TeamsIdMemberships
        |
        |=> username -> TeamsIdMembershipsUsername = username_str
    @TeamsIdRepos
        |
        |=> org -> TeamsIdReposOrg = org_str
    @TeamsIdReposOrg
        |
        |=> repo -> TeamsIdReposOrgRepo = repo_str
);

exec!(TeamsIdMembershipsUsername);
exec!(TeamsIdReposOrgRepo);
