//! Access the (notifications)[https://developer.github.com/v3/activity/notifications/]
//! portion of the GitHub API
imports!();
use client::PatchQueryBuilder;

new_type!(
    Notifications
    NotificationsThreads
);

from!(
    @PatchQueryBuilder
       -> Notifications = "notifications"
    @Notifications
       -> NotificationsThreads = "threads"
    @NotificationsThreads
       => NotificationsThreadsId
);

impl_macro!(
    @Notifications
        |=> threads -> NotificationsThreads
        |
    @NotificationsThreads
       |
       |=> id -> NotificationsThreadsId = thread_id
);

exec!(NotificationsThreadsId);
