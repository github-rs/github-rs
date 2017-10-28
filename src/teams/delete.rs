//! Access the Teams portion of the Github API
imports!();
use client::DeleteQueryBuilder;

new_type!(
    Teams
    TeamsId
    TeamsIdMemberships
    TeamsIdRepos
    TeamsIdReposOwner
    TeamsIdReposOwnerRepo
    TeamsIdMembershipsUsername
);

from!(
    @DeleteQueryBuilder
        -> Teams = "teams"
    @Teams
        => TeamsId
    @TeamsId
        -> TeamsIdMemberships = "memberships"
        -> TeamsIdRepos = "repos"
    @TeamsIdRepos
        => TeamsIdReposOwner
    @TeamsIdReposOwner
        => TeamsIdReposOwnerRepo
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
        |=> owner -> TeamsIdReposOwner = owner_str
    @TeamsIdReposOwner
        |
        |=> repo -> TeamsIdReposOwnerRepo = repo_str
);

exec!(TeamsId);
exec!(TeamsIdMembershipsUsername);
exec!(TeamsIdReposOwnerRepo);
