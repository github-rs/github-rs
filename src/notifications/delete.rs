//! Access the (notifications)[https://developer.github.com/v3/activity/notifications/]
//! portion of the GitHub API
imports!();
use client::DeleteQueryBuilder;

new_type!(
    Notifications
    NotificationsThreads
    NotificationsThreadsId
);

from!(
    @DeleteQueryBuilder
       -> Notifications = "notifications"
    @Notifications
       -> NotificationsThreads = "threads"
    @NotificationsThreads
       => NotificationsThreadsId
    @NotificationsThreadsId
       -> NotificationsThreadsIdSubscription = "subscription"
);

impl_macro!(
    @Notifications
        |=> threads -> NotificationsThreads
        |
    @NotificationsThreads
       |
       |=> id -> NotificationsThreadsId = thread_id
    @NotificationsThreadsId
       |=> subscription -> NotificationsThreadsIdSubscription
       |
);

exec!(NotificationsThreadsIdSubscription);
