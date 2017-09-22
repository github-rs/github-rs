//! Access the Users portion of the GitHub API
imports!();
use client::GetQueryBuilder;

// Declaration of types representing the various items under users
new_type!(
    Emails
    Followers
    Following
    FollowingUser
    Keys
    KeysId
    Orgs
    User
    Users
    UsersOrgs
    Events
    EventsOrgs
    EventsOrgsName
    Gists
    UsersKeys
    UsersStarred
    UserUsername
    UsersUsername
    Repos
    ReceivedEvents
    Issues
    Public
    Starred
    StarredRepo
    StarredOwner
    Subscriptions
);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> User  = "user"
        -> Users = "users"
    @Events
        -> EventsOrgs = "orgs"
        -> Public =  "public"
    @EventsOrgs
        => EventsOrgsName
    @Following
        => FollowingUser
    @Keys
        => KeysId
    @Starred
        => StarredOwner
    @StarredOwner
        => StarredRepo
    @User
        -> Emails = "emails"
        -> Followers = "followers"
        -> Following = "following"
        -> Keys = "keys"
        -> Issues = "issues"
        -> Orgs = "orgs"
        -> Repos = "repos"
        -> Subscriptions = "subscriptions"
        -> Starred = "starred"
    @Users
        => UsersUsername
    @UserUsername
        -> Followers = "followers"
        -> Following = "following"
        -> UsersKeys = "keys"
        -> Repos = "repos"
    @UsersUsername
        -> Followers = "followers"
        -> Following = "following"
        -> Events = "events"
        -> Gists = "gists"
        -> UsersOrgs = "orgs"
        -> UsersKeys = "keys"
        -> Repos = "repos"
        -> Subscriptions = "subscriptions"
        -> UsersStarred = "starred"
        -> ReceivedEvents = "received_events"
);

// impls of each type
impl_macro!(
    @Starred
        |
        |=> owner -> StarredOwner = owner_str
    @StarredOwner
        |
        |=> repo -> StarredRepo = repo_str
    @User
        |=> emails -> Emails
        |=> followers -> Followers
        |=> following -> Following
        |=> issues -> Issues
        |=> repos -> Repos
        |=> subscriptions -> Subscriptions
        |=> starred -> Starred
        |=> keys -> Keys
        |=> orgs -> Orgs
        |
    @Users
        |
        |=> username -> UsersUsername = username_str
    @UserUsername
        |=> followers -> Followers
        |=> following -> Following
        |=> keys -> UsersKeys
        |=> repos -> Repos
        |
    @UsersUsername
        |=> events -> Events
        |=> followers -> Followers
        |=> following -> Following
        |=> gists -> Gists
        |=> orgs -> UsersOrgs
        |=> keys -> UsersKeys
        |=> received_events -> ReceivedEvents
        |=> repos -> Repos
        |=> starred -> UsersStarred
        |=> subscriptions -> Subscriptions
        |
    @Events
        |=> orgs -> EventsOrgs
        |=> public -> Public
        |
    @EventsOrgs
        |
        |=> org -> EventsOrgsName = org_name_str
    @Keys
        |
        |=> id -> KeysId = id_str
    @Following
        |
        |=> username -> Following = username_str
);

exec!(Emails);
exec!(Events);
exec!(EventsOrgsName);
exec!(Followers);
exec!(Following);
exec!(FollowingUser);
exec!(Gists);
exec!(Issues);
exec!(Keys);
exec!(KeysId);
exec!(Orgs);
exec!(Public);
exec!(ReceivedEvents);
exec!(Repos);
exec!(Starred);
exec!(StarredRepo);
exec!(Subscriptions);
exec!(User);
exec!(UserUsername);
exec!(Users);
exec!(UsersKeys);
exec!(UsersOrgs);
exec!(UsersStarred);
exec!(UsersUsername);
