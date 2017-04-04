//! Helper functions for end users for GitHub response Headers
use std::str;
use std::str::FromStr;
use hyper::header::{ ETag, Headers, LastModified, UserAgent };

/// Checks to see if a received payload from GitHub contains
/// the GitHub-Hookshot header in the `UserAgent`.
pub fn has_github_hookshot(head: &Headers) -> bool {
    head.get::<UserAgent>()
        .map_or(false, |user_agent| {
            match user_agent {
                &UserAgent(ref raw) => raw.starts_with("GitHub-Hookshot"),
            }
        })
}

/// Extract an ETag from the Headers if it exists
pub fn etag(head: &Headers) -> Option<ETag> {
    head.get::<ETag>()
        .map(|tag| tag.to_owned())
}

/// Extract the Last-Modified from the Headers if it exists
pub fn last_modified(head: &Headers) -> Option<LastModified> {
    head.get::<LastModified>()
        .map(|modified| modified.to_owned())
}


/// Extract however many requests the authenticated user can
/// do from the Headers
pub fn rate_limit_remaining(head: &Headers) -> Option<u32> {
    head.get_raw("X-RateLimit-Remaining")
        .map(|limit| {
            u32::from_str(
                str::from_utf8(limit.one()
                                    .unwrap_or(&[]))
                .unwrap_or("")
            ).ok()
        })
        .unwrap_or(None)
}

/// Extract however many requests the authenticated user can
/// make from the Headers
pub fn rate_limit(head: &Headers) -> Option<u32> {
    head.get_raw("X-RateLimit-Limit")
        .map(|limit| {
            u32::from_str(
                str::from_utf8(limit.one()
                                    .unwrap_or(&[]))
                .unwrap_or("")
            ).ok()
        })
        .unwrap_or(None)
}

/// Extract when the requests limit for the authenticated user
/// is reset from the Headers
pub fn rate_limit_reset(head: &Headers) -> Option<u32> {
    head.get_raw("X-RateLimit-Reset")
        .map(|limit| {
            u32::from_str(
                str::from_utf8(limit.one()
                                    .unwrap_or(&[]))
                .unwrap_or("")
            ).ok()
        })
        .unwrap_or(None)
}
