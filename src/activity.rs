use github::Client;
use json::Event;
use requests::*;
use error::*;
use serde_json;

/// Trait used to define access to endpoints grouped under `Activity` in the Github API
/// specification
pub trait Activity {
    // Events
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /events
    /// ### Description
    /// Returns a vector of recent `Event`s from the API
    fn get_events(&self) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/events
    /// ### Description
    /// Returns a vector of `Event`s from the repo of a given owner
    fn get_repos_owner_repo_events(&self, owner: &str, repo: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/issues/events
    /// ### Description
    /// Returns a vector of `Event`s from the issues of a repo of a given owner
    fn get_repos_owner_repo_issues_events(&self, owner: &str, repo: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /networks/:owner/:repo/events
    /// ### Description
    /// Returns a vector of `Event`s from the network of a repo of a given owner
    fn get_networks_owner_repo_events(&self, owner: &str, repo: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /orgs/:org/events
    /// ### Description
    /// Returns a vector of `Event`s from a given organization
    fn get_orgs_org_events(&self, org: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/received_events
    /// ### Description
    /// Returns a vector of `Event`s the username has received. If you are authenticated and use
    /// your own username you will see private `Event`s
    fn get_users_username_received_events(&self, username: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/received_events/public
    /// ### Description
    /// Returns a vector of public `Event`s the username has received.
    fn get_users_username_received_events_public(&self, username: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/events
    /// ### Description
    /// Returns a vector of `Event`s performed by the username. If you are authenticated and use
    /// your own username you will see private `Event`s
    fn get_users_username_events(&self, username: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/events/public
    /// ### Description
    /// Returns a vector of `Event`s performed by the username. If you are authenticated and use
    /// your own username you will see private `Event`s
    fn get_users_username_events_public(&self, username: &str) -> Result<Vec<Event>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/events/orgs/:org
    /// ### Description
    /// Returns a vector of `Event`s from the username's org. You must be authenticated to view
    /// this.
    fn get_users_username_events_orgs_org(&self, username: &str, org: &str) -> Result<Vec<Event>>;
}

// Doc comments are duplicated so that regardless of trait or client they see how to use it
impl Activity for Client {
    // EVENTS TODO UPDATE TO INCLUDE PAGINATION
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /events
    /// ### Description
    /// Returns a vector of recent `Event`s from the API
    fn get_events(&self) -> Result<Vec<Event>> {
        let url = "https://api.github.com/events";
        let data = get(url, self.get_headers().clone())?;

        // If the auth token is wrong we want this to panic.
        try_serde!(serde_json::from_str(&data))

    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/events
    /// ### Description
    /// Returns a vector of `Event`s from the repo of a given owner
    fn get_repos_owner_repo_events(&self, owner: &str, repo: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/repos/");
        url.push_str(owner);
        url.push('/');
        url.push_str(repo);
        url.push_str("/events");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /repos/:owner/:repo/issues/events
    /// ### Description
    /// Returns a vector of `Event`s from the issues of a repo of a given owner
    fn get_repos_owner_repo_issues_events(&self, owner: &str, repo: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/repos/");
        url.push_str(owner);
        url.push('/');
        url.push_str(repo);
        url.push_str("/issues/events");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /networks/:owner/:repo/events
    /// ### Description
    /// Returns a vector of `Event`s from the network of a repo of a given owner
    fn get_networks_owner_repo_events(&self, owner: &str, repo: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/networks/");
        url.push_str(owner);
        url.push('/');
        url.push_str(repo);
        url.push_str("/events");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /orgs/:org/events
    /// ### Description
    /// Returns a vector of `Event`s from a given organization
    fn get_orgs_org_events(&self, org: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/orgs/");
        url.push_str(org);
        url.push_str("/events");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))

    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/received_events
    /// ### Description
    /// Returns a vector of `Event`s the username has received. If you are authenticated and use
    /// your own username you will see private `Event`s
    fn get_users_username_received_events(&self, username: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/received_events");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))

    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/received_events/public
    /// ### Description
    /// Returns a vector of public `Event`s the username has received.
    fn get_users_username_received_events_public(&self, username: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/received_events/public");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/events
    /// ### Description
    /// Returns a vector of `Event`s performed by the username. If you are authenticated and use
    /// your own username you will see private `Event`s
    fn get_users_username_events(&self, username: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/events");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/events/public
    /// ### Description
    /// Returns a vector of `Event`s performed by the username. If you are authenticated and use
    /// your own username you will see private `Event`s
    fn get_users_username_events_public(&self, username: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/events/public");
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/events/orgs/:org
    /// ### Description
    /// Returns a vector of `Event`s from the username's org. You must be authenticated to view
    /// this.
    fn get_users_username_events_orgs_org(&self, username: &str, org: &str) -> Result<Vec<Event>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/events/orgs/");
        url.push_str(org);
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))

    }
}
