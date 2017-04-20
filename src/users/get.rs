//! Access the Users portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

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
    @Emails
        => Executor
    @Events
        => Executor
        -> EventsOrgs = "orgs"
        -> Public =  "public"
    @EventsOrgs
        => EventsOrgsName
        => Executor
    @EventsOrgsName
        => Executor
    @Followers
        => Executor
    @Following
        => FollowingUser
        => Executor
    @FollowingUser
        => Executor
    @Gists
        => Executor
    @Issues
        => Executor
    @Keys
        => KeysId
        => Executor
    @KeysId
        => Executor
    @Orgs
        => Executor
    @Public
        => Executor
    @ReceivedEvents
        => Executor
    @Subscriptions
        => Executor
    @Starred
        => Executor
        => StarredOwner
    @StarredOwner
        => StarredRepo
    @StarredRepo
        => Executor
    @User
        => Executor
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
        => Executor
        => UsersUsername
    @UsersOrgs
        => Executor
    @UsersKeys
        => Executor
    @UsersStarred
        => Executor
    @UserUsername
        => Executor
        -> Followers = "followers"
        -> Following = "following"
        -> UsersKeys = "keys"
        -> Repos = "repos"
    @UsersUsername
        => Executor
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
    @Repos
        => Executor
);

// impls of each type
impl_macro!(
    @Starred
        |
        |=> owner -> StarredOwner = owner_str
        |-> execute
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
        |-> execute
    @Users
        |
        |=> username -> UsersUsername = username_str
        |-> execute
    @UserUsername
        |=> followers -> Followers
        |=> following -> Following
        |=> keys -> UsersKeys
        |=> repos -> Repos
        |
        |-> execute
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
        |-> execute
    @Events
        |=> orgs -> EventsOrgs
        |=> public -> Public
        |
        |-> execute
    @EventsOrgs
        |
        |=> org -> EventsOrgsName = org_name_str
    @Keys
        |
        |=> id -> KeysId = id_str
        |-> execute
    @Following
        |
        |=> username -> Following = username_str
        |-> execute
    @ReceivedEvents
        |
        |-> execute
    @UsersKeys
        |
        |-> execute
    @Emails
        |
        |-> execute
    @EventsOrgsName
        |
        |-> execute
    @FollowingUser
        |
        |-> execute
    @Gists
        |
        |-> execute
    @Issues
        |
        |-> execute
    @KeysId
        |
        |-> execute
    @Followers
        |
        |-> execute
    @Repos
        |
        |-> execute
    @Subscriptions
        |
        |-> execute
    @StarredRepo
        |
        |-> execute
    @Orgs
        |
        |-> execute
    @Public
        |
        |-> execute
    @UsersOrgs
        |
        |-> execute
    @UsersStarred
        |
        |-> execute
);
