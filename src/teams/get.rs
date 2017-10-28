//! Access the Teams portion of the Github API
imports!();
use client::GetQueryBuilder;

new_type!(
    Teams
    TeamsId
    TeamsIdMemberships
    TeamsIdMembershipsUsername
    TeamsIdInvitations
    TeamsIdMembers
    TeamsIdRepos
    TeamsIdReposOwner
    TeamsIdReposOwnerRepo
);

from!(
    @GetQueryBuilder
        -> Teams = "teams"
    @Teams
        => TeamsId
    @TeamsId
        -> TeamsIdMemberships = "memberships"
        -> TeamsIdInvitations = "invitations"
        -> TeamsIdMembers = "members"
        -> TeamsIdRepos = "repos"
    @TeamsIdMemberships
        => TeamsIdMembershipsUsername
    @TeamsIdRepos
        => TeamsIdReposOwner
    @TeamsIdReposOwner
        => TeamsIdReposOwnerRepo
);

impl_macro!(
    @Teams
        |
        |=> id -> TeamsId = id_str
    @TeamsId
        |=> memberships -> TeamsIdMemberships
        |=> invitations -> TeamsIdInvitations
        |=> members -> TeamsIdMembers
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
exec!(TeamsIdInvitations);
exec!(TeamsIdRepos);
exec!(TeamsIdMembers);
exec!(TeamsIdMembershipsUsername);
exec!(TeamsIdReposOwnerRepo);
