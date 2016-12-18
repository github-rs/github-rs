//! Structs related to JSON serialization/deserialization
#![allow(missing_docs)]
extern crate serde;
extern crate serde_json;

/// Information related to a user on Github is stored in this struct.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    // Public
    // These will all have a value of some sort for any
    // user
    pub id: u64,
    pub avatar_url: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,

    // Optional Public
    // These are optional for a user to fill in and so
    // there might not be a value to have.
    pub login: Option<String>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<String>,
    pub bio: Option<String>,

    // These values also may or may not be available depending
    // on the user endpoint used. Accessing the username directly
    // or the authenticated user themselves will get these fields.
    pub public_repos: Option<u64>,
    pub public_gists: Option<u64>,
    pub followers: Option<u64>,
    pub following: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    // Private User
    // The fields below here are only accesible if looking at your
    // own profile using an AuthToken. Otherwise these fields will
    // have None for users other than yourself.
    pub private_gists: Option<u64>,
    pub total_private_repos: Option<u64>,
    pub owned_private_repos: Option<u64>,
    pub disk_usage: Option<u64>,
    pub collaborators: Option<u64>,
    pub plan: Option<Plan>,
    #[serde(rename="type")]
    pub type_: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub space: u64,
    pub collaborators: u64,
    pub private_repos: u64,
}

/// Used to update an authenticated user.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchUser {
    pub name: String,
    pub email: String,
    pub blog: String,
    pub company: String,
    pub location: String,
    pub hireable: bool,
    pub bio: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub verifiable_password_authentication: bool,
    pub github_services_sha: String,
    pub hooks: Vec<String>,
    pub git: Vec<String>,
    pub pages: Vec<String>,
    pub importer: Vec<String>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct RateLimit {
    pub resources: Resources,
    pub rate: Rate,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Resources {
    pub core: Core,
    pub search: Search,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Rate {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Search {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Core {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Email {
    pub email: String,
    pub verified: bool,
    pub primary: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RepoCreate {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub private: Option<bool>,
    pub has_issues: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_downloads: Option<bool>,
    pub team_id: Option<i32>,
    pub auto_init: Option<bool>,
    pub gitignore_template: Option<String>,
    pub license_template: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SSHKey {
    pub id: u64,
    pub key: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub verified: Option<bool>,
    pub created_at: Option<String>,
    pub read_only: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GPGKey {
    pub id: u64,
    pub primary_key_id: Option<u64>,
    pub key_id: String,
    pub public_key: String,
    pub emails: Vec<Email>,
    pub subkeys: Option<Vec<GPGKey>>,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GPGKeyPost {
    pub armored_public_key: String,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GitIgnore {
    pub name: String,
    pub source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Markdown {
    pub text: String,
    pub mode: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename="type")]
    pub type_: String,
    pub public: bool,
    pub payload: Vec<String>, // Correct type?
    pub repo: Repo,
    pub actor: Actor,
    pub org: Org,
    pub created_at: String,
    pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Org {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

// Git Data Types
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Blob {
    pub content: String,
    pub encoding: String,
    pub url: String,
    pub sha: String,
    pub size: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobCreate {
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub url: String,
    pub author: Author,
    pub committer: Committer,
    pub message: String,
    pub tree: Tree,
    pub parents: Vec<Parent>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub date: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Committer {
    pub date: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    pub url: String,
    pub sha: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Parent {
    pub url: String,
    pub sha: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ref {
    #[serde(rename="ref")]
    pub ref_: String,
    pub url: String,
    pub object: Object,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Object {
    #[serde(rename="type")]
    pub type_: String,
    pub sha: String,
    pub url: String,
}
