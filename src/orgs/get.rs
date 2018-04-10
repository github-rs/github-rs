//! Access the Organizations portion of the Github API
imports!();
use client::GetQueryBuilder;

new_type!(
    OrgsOrgMemberships
);

from!(
    @GetQueryBuilder
        -> Orgs = "orgs"
    @Orgs
        => OrgsOrg
    @OrgsOrg
        -> OrgsOrgEvents = "events"
        -> OrgsOrgHooks = "hooks"
        -> OrgsOrgInvitations = "invitations"
        -> OrgsOrgIssues = "issues"
        -> OrgsOrgMembers = "members"
        -> OrgsOrgOutsidecollaborators = "outside_collaborators"
        -> OrgsOrgPublicmembers = "public_members"
        -> OrgsOrgRepos = "repos"
        -> OrgsOrgTeams = "teams"
    @OrgsOrgHooks
        => OrgsOrgHooksId
    @OrgsOrgMembers
        => OrgsOrgMembersUsername
    @OrgsOrgMemberships
        => OrgsOrgMembershipsUsername
    @OrgsOrgPublicmembers
        => OrgsOrgPublicmembersUsername
);

impl_macro!(
    @Orgs
        |
        |=> org -> OrgsOrg = org_str
    @OrgsOrg
        |=> events -> OrgsOrgEvents
        |=> hooks -> OrgsOrgHooks
        |=> invitations -> OrgsOrgInvitations
        |=> issues -> OrgsOrgIssues
        |=> members -> OrgsOrgMembers
        |=> outside_collaborators -> OrgsOrgOutsidecollaborators
        |=> public_members -> OrgsOrgPublicmembers
        |=> repos -> OrgsOrgRepos
        |=> teams -> OrgsOrgTeams
        |
    @OrgsOrgHooks
        |
        |=> id -> OrgsOrgHooksId = org_hook_id_str
    @OrgsOrgMembers
        |
        |=> username -> OrgsOrgMembersUsername = org_members_username_str
    @OrgsOrgMemberships
        |
        |=> username -> OrgsOrgMembershipsUsername = org_memberships_username_str
    @OrgsOrgPublicmembers
        |
        |=> username -> OrgsOrgPublicmembersUsername = org_public_members_username
);

exec!(Orgs);
exec!(OrgsOrg);
exec!(OrgsOrgEvents);
exec!(OrgsOrgHooks);
exec!(OrgsOrgHooksId);
exec!(OrgsOrgInvitations);
exec!(OrgsOrgIssues);
exec!(OrgsOrgMembers);
exec!(OrgsOrgMembersUsername);
exec!(OrgsOrgMembershipsUsername);
exec!(OrgsOrgOutsidecollaborators);
exec!(OrgsOrgPublicmembers);
exec!(OrgsOrgPublicmembersUsername);
exec!(OrgsOrgRepos);
exec!(OrgsOrgTeams);
