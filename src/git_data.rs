//! Trait definition related to Git Data on Github

use serde_json;
use json::{Commit,Ref}; //Blob, BlobCreate
use requests::*;
use github::Client;
use error::*;

/// Trait used to define access to endpoints grouped under `Users` in the Github API
/// specification
pub trait GitData {

    // /// ### Request Type:
    // /// `GET`
    // /// ### Endpoint:
    // /// /repos/:owner/:repo/git/blobs/:sha
    // /// ### Description
    // /// Returns a `Blob` Struct for the requested owner's repo and sha.
    // fn get_blob(&self, owner: &str, repo: &str, sha: &str) -> Result<Blob>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/git/commits/:sha
    /// ### Description
    /// Returns a `Blob` Struct for the requested owner's repo and sha.
    fn get_commit(&self, owner: &str, repo: &str, sha: &str) -> Result<Commit>;

    // This requires a repo so get to it at some point
    // /// ### Request Type:
    // /// `POST`
    // /// ### Endpoint:
    // /// /repos/:owner/:repo/git/commits
    // /// ### Description
    // /// Creates a commit for an owner's repo and returns the commit if it
    // /// succeeded.
    // fn post_commit(&self, owner: &str, repo: &str) -> Result<Commit>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/git/refs/:ref
    /// ### Description
    /// Returns a `Ref` Struct for the requested owner's repo and sha.
    fn get_ref(&self, owner: &str, repo: &str, _ref: &str) -> Result<Ref>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/git/refs/:ref
    /// ### Description
    /// Returns an array of `Ref` Struct for the requested owner's repo and sha.
    /// This should be used if there are multiple refs. If you have two branches
    /// projectA and projectB and use "project" as the input to `_ref` then you'll
    /// need this as opposed to get_ref since you'll get the ref for both projectA
    /// and projectB.
    fn get_ref_mult(&self, owner: &str, repo: &str, _ref: &str) -> Result<Vec<Ref>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/git/refs
    /// ### Description
    /// Returns a vector containing all `Ref` Struct for the requested owner's repo and sha.
    fn get_refs(&self, owner: &str, repo: &str) -> Result<Vec<Ref>>;
}

impl GitData for Client {

    // /// This gets 301 errors right now and shouldn't be used yet
    // fn get_blob(&self, owner: &str, repo: &str, sha: &str) -> Result<Blob> {
    //     let url = format!(
    //             "https://api.github.com/repos/{}/{}/git/blobs/{}",
    //             owner, repo, sha
    //         );
    //     let data = get(&url, self.headers.clone())?;
    //     try_serde!(serde_json::from_str(&data))
    // }

    /// Retrieves a specific commit from an owner's repo given the commit hash
    fn get_commit(&self, owner: &str, repo: &str, sha: &str) -> Result<Commit> {
        let url = format!(
                "https://api.github.com/repos/{}/{}/git/commits/{}",
                owner, repo, sha
            );
        let data = get(&url, self.headers.clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    /// Retrieves a specific ref from an owner's repo given the ref
    fn get_ref(&self, owner: &str, repo: &str, _ref: &str) -> Result<Ref> {
        let url = format!(
                "https://api.github.com/repos/{}/{}/git/refs/{}",
                owner, repo, _ref
            );
        let data = get(&url, self.headers.clone())?;

        if data.contains("Not Found") {
            Err(GithubError::Github404)
        } else {
            try_serde!(serde_json::from_str(&data))
        }
    }

    /// Retrieves multiple refs based on a partial match from an owner's repo given the ref
    fn get_ref_mult(&self, owner: &str, repo: &str, _ref: &str) -> Result<Vec<Ref>> {
        let url = format!(
                "https://api.github.com/repos/{}/{}/git/refs/{}",
                owner, repo, _ref
            );
        let data = get(&url, self.headers.clone())?;
        if data.contains("Not Found") {
            Err(GithubError::Github404)
        } else {
            try_serde!(serde_json::from_str(&data))
        }
    }

    /// Retrieves all refs an owner's repo given the co
    fn get_refs(&self, owner: &str, repo: &str) -> Result<Vec<Ref>> {
        let url = format!(
                "https://api.github.com/repos/{}/{}/git/refs",
                owner, repo
            );
        let data = get(&url, self.headers.clone())?;
        if data.contains("Not Found") {
            Err(GithubError::Github404)
        } else {
            try_serde!(serde_json::from_str(&data))
        }
    }
}
