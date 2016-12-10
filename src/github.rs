//! Docs for code used to access the Github API in Rust
//! # Github Reference Docs
//! - [Activity](https://developer.github.com/v3/activity)
//!     - [Events](https://developer.github.com/v3/activity/events)
//!     - [Event Type and Payloads](https://developer.github.com/v3/activity/events/types)
//!     - [Feeds](https://developer.github.com/v3/activity/feeds)
//!     - [Starring](https://developer.github.com/activity/starring)
//!     - [Watching](https://developer.github.com/activity/watching)
//! - [Gists](https://developer.github.com/v3/gists)
//!     - [Comments](https://developer.github.com/v3/gists/comments)
//! - [Git Data](https://developer.github.com/v3/git)
//!     - [Blobs](https://developer.github.com/v3/git/blobs)
//!     - [Commits](https://developer.github.com/v3/git/commits)
//!     - [References](https://developer.github.com/v3/git/refs)
//!     - [Tags](https://developer.github.com/v3/git/tags)
//!     - [Trees](https://developer.github.com/v3/git/trees)
//! - [Issues](https://developer.github.com/v3/issues)
//!     - [Assignees](https://developer.github.com/v3/issues/)
//!     - [Comments](https://developer.github.com/v3/issues/)
//!     - [Events](https://developer.github.com/v3/issues/)
//!     - [Labels](https://developer.github.com/v3/issues/)
//!     - [Milestones](https://developer.github.com/v3/issues/)
//!     - [Timeline](https://developer.github.com/v3/issues/)
//! - [Migration](https://developer.github.com/v3/migration)
//!     - [Migrations](https://developer.github.com/v3/migration/migrations)
//!     - [Source Imports](https://developer.github.com/v3/migration/source_imports)
//! - [Miscellaneous](https://developer.github.com/v3/misc)
//!     - [Emojis](https://developer.github.com/v3/emojis)
//!     - [Gitignore](https://developer.github.com/v3/gitignore)
//!     - [Licenses](https://developer.github.com/v3/licenses)
//!     - [Markdown](https://developer.github.com/v3/markdown)
//!     - [Meta](https://developer.github.com/v3/meta)
//!     - [Rate Limit](https://developer.github.com/v3/rate_limit)
//! - [Organizations](https://developer.github.com/v3/orgs)
//!     - [Members](https://developer.github.com/v3/orgs/members)
//!     - [Teams](https://developer.github.com/v3/orgs/teams)
//!     - [Webhooks](https://developer.github.com/v3/orgs/hooks)
//! - [Pull Requests](https://developer.github.com/v3/pulls)
//!     - [Review Comments](https://developer.github.com/v3/pulls/comments)
//! - [Reactions](https://developer.github.com/v3/reactions)
//! - [Repositories](https://developer.github.com/v3/repos)
//!     - [Collaborators](https://developer.github.com/v3/repos/collaborators)
//!     - [Comments](https://developer.github.com/v3/repos/comments)
//!     - [Commits](https://developer.github.com/v3/repos/commits)
//!     - [Contents](https://developer.github.com/v3/repos/contents)
//!     - [Deploy Keys](https://developer.github.com/v3/repos/keys)
//!     - [Deployments](https://developer.github.com/v3/repos/deployments)
//!     - [Forks](https://developer.github.com/v3/repos/forks)
//!     - [Merging](https://developer.github.com/v3/repos/merging)
//!     - [Pages](https://developer.github.com/v3/repos/pages)
//!     - [Releases](https://developer.github.com/v3/repos/releases)
//!     - [Statistics](https://developer.github.com/v3/repos/statistics)
//!     - [Statuses](https://developer.github.com/v3/repos/statuses)
//!     - [Webhooks](https://developer.github.com/v3/repos/hooks)
//! - [Search](https://developer.github.com/v3/search)
//! - [Users](https://developer.github.com/v3/users)
//!     - [Emails](https://developer.github.com/v3/users/emails)
//!     - [Followers](https://developer.github.com/v3/users/followers)
//!     - [SSH Keys](https://developer.github.com/v3/users/keys)
//!     - [GPG Keys](https://developer.github.com/v3/users/gpg_keys)
//!     - [Administration](https://developer.github.com/v3/users/administration)
//! - [Enterprise](https://developer.github.com/v3/enterprise)
//!     - [Admin Stats](https://developer.github.com/v3/enterprise/admin_stats)
//!     - [LDAP](https://developer.github.com/v3/enterprise/ldap)
//!     - [License](https://developer.github.com/v3/enterprise/license)
//!     - [Management Console](https://developer.github.com/v3/enterprise/management_console)
//!     - [Search Indexing](https://developer.github.com/enterprise/search_indexing)
//!     - [Organization Administration](https://developer.github.com/enterprise/orgs)
//!
//!
//! # Design
//! By abstracting away the code underlying the calls into a single
//! struct via Traits, it makes it easy to make API calls without having
//! to import all of the different modules to do it. Merely create a new
//! struct and make the calls from it.
//!
//! The different traits represent all of the different
//! methods available to access the API and what they do. Since all of
//! them are implemented for this struct all of the methods are available
//! on this page and there is no need to import any of the other files
//! ever.
//! While all the trait implementations and definitions reside in different files within the
//! codebase itself they've all been imported into this module. You'll be able to make any calls
//! you need with the Github struct and not need to worry about trait imports or the like.
//!
//! ## Examples
//!
//! ```ignore
//! extern crate github_rs;
//! // Imports trait methods and everything else automatically
//! use github_rs::github::*;
//! let github = Client::new("Your API Token Here");
//! println!("{:#?}", github.get_user());
//! ```
//!

extern crate hyper;
use hyper::header::Headers;

// Under no circumstances do we want this exposed. This is the internal library used to build calls
// to the API and it contains some things needed for other modules to work.
use requests::*;

// Imports for all of the traits that contain the calls to the API
// that are used by the Github struct. This hides the implementation and puts all of the
// documentation as part of one module. This is exactly what we want. We don't want the user to
// know how requests are made. We don't want them to know where the traits are implemented. We just
// want them to know they've been logically grouped and that they can make calls real nice and
// easy. No work for them.
pub use activity::Activity;
pub use git_data::GitData;
// pub use enterprise::Enterprise;
// pub use gists::Gists;
// pub use issues::Issues;
// pub use migration::Migration;
pub use miscellaneous::Misc;
// pub use oauth::Oauth;
// pub use organizations::Organizations;
// pub use pull_requests::Pull_Requests;
// pub use reactions::Reactions;
pub use repositories::Repos;
// pub use search::Search;
pub use types::*;
pub use users::Users;

// We want people to see the structs. By re-exporting it here, the actual calls are obscured and
// it's just the thing returned. JSON doesn't even have to be touched by the user directly
pub use json::*;
pub use error::*;

/// Struct used to make calls to the Github API.
#[derive(Debug, Clone)]
pub struct Client {
    /// Token generated from [here on Github](https://github.com/settings/tokens)
    /// that grants the `Client` access to various parts of the API
    /// depending on what is allowed with that token.
    pub token: AccessToken,
    /// Headers that are automatically defined and used in API Calls to the GithubAPI via
    /// the new function. If you wish to define your own to change how you make calls you should
    /// modify this variable. However, this is discouraged as the automatically created Headers
    /// contain the `Authorization`, API Version, Media type, and `UserAgent` headers for you.
    /// If you must do so it might cause undefined behavior for the API and errors encountered are
    /// not tested for and not the fault of the library authors
    pub headers: Headers,
}

impl Client {
    /// Generate a new Client struct with the access token so that
    /// calls can be made to the API using it
    pub fn new(acc_token: AccessToken) -> Result<Client> {
        let cloned = acc_token.clone();
        Ok(Client {
            token: acc_token,
            headers: default_headers(cloned)?,
        })
    }
}
