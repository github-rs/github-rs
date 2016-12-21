//! Trait definition related to Miscellaneous items on Github
use json::{GitIgnore, RateLimit, Meta, Markdown};
use requests::*;
use error::*;
use types::HTML;
use github::Client;
use serde_json;

/// Trait used to specify function hearders for endpoints grouped under Miscellaneous in the Github
/// API specification
pub trait Misc {
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /meta
    /// ### Description
    /// Returns a `Meta` struct which contains information about the Github service
    fn get_meta(&self) -> Result<Meta>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /rate_limit
    /// ### Description
    /// Returns a RateLimit struct to allow the user to know how many requests they can still make.
    /// Note hiting this endpoint with a request does not count against that limit.
    fn get_rate_limit(&self) -> Result<RateLimit>;

    /// ### Request Type:
    /// GET
    /// ### Endpoint:
    /// /gitignore/templates
    /// ### Description
    /// Returns a vector of the languages that have gitignore templates on Github.
    fn get_gitignore_templates(&self) -> Result<Vec<String>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /gitignore/templates/:lang
    /// ### Description
    /// Returns a `GitIgnore` struct that has the string for the gitignore file and the language as
    /// part of it.
    fn get_gitignore_templates_lang(&self, lang: &str) -> Result<GitIgnore>;

    // Getting this to work with a Struct will take a while to get
    // all the available emojis. Not Impossible just hard.
    // fn get_emojis(&self) -> String;

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /markdown
    /// ### Description
    /// Returns a rendered version of the `Markdown` sent to Github as `HTML`.
    fn post_markdown(&self, data: Markdown) -> Result<HTML>;

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /markdown/raw
    /// ### Description
    /// Returns a rendered version of the markdown posted to it. This expects the request to not be
    /// JSON so right now this function is broken until it can be updated to work properly.
    fn post_markdown_raw(&self, data: Markdown) -> Result<HTML>;
}

// Doc comments are duplicated so that regardless of trait or client they see how to use it
/// Implementation of the `Misc` trait for the `Client`
impl Misc for Client {
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /meta
    /// ### Description
    /// Returns a `Meta` struct which contains information about the Github service
    fn get_meta(&self) -> Result<Meta> {
        let url = "https://api.github.com/meta";
        let data = get(url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /rate_limit
    /// ### Description
    /// Returns a RateLimit struct to allow the user to know how many requests they can still make.
    /// Note hiting this endpoint with a request does not count against that limit.
    fn get_rate_limit(&self) -> Result<RateLimit> {
        let url = "https://api.github.com/rate_limit";
        let data = get(url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// GET
    /// ### Endpoint:
    /// /gitignore/templates
    /// ### Description
    /// Returns a vector of the languages that have gitignore templates on Github.
    fn get_gitignore_templates(&self) -> Result<Vec<String>> {
        let url = "https://api.github.com/gitignore/templates";
        let data = get(url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /gitignore/templates/:lang
    /// ### Description
    /// Returns a `GitIgnore` struct that has the string for the gitignore file and the language as
    /// part of it.
    // Does not implement raw media type yet
    fn get_gitignore_templates_lang(&self, lang: &str) -> Result<GitIgnore> {
        let mut url = String::from("https://api.github.com/gitignore/templates/");
        url.push_str(lang);
        let data = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /markdown
    /// ### Description
    /// Returns a rendered version of the `Markdown` sent to Github as `HTML`.
    fn post_markdown(&self, data: Markdown) -> Result<HTML> {
        let url = "https://api.github.com/markdown";
        let data = post(url,
                        self.get_headers().clone(),
                        serde_json::to_string(&data)?)?;
        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /markdown/raw
    /// ### Description
    /// Returns a rendered version of the markdown posted to it. This expects the request to not be
    /// JSON so right now this function is broken until it can be updated to work properly.
    // Broken not sure how to fix right now. This also expects
    // the datatype to be raw not JSON
    fn post_markdown_raw(&self, data: Markdown) -> Result<HTML> {
        let url = "https://api.github.com/markdown/raw";
        let data = post(url,
                        self.get_headers().clone(),
                        serde_json::to_string(&data)?)?;
        try_serde!(serde_json::from_str(&data))
    }

    // fn get_emojis(&self) -> String {
    //     let url = "https://api.github.com/emojis";
    //     let data = get(url, self.get_headers().clone())
    // }

    // Experimental come back when stabilized
    // GET /licenses
    // GET /licenses/mit
    // GET /repos/:owner/:repo
    // GET /repos/:owner/:repo/:license
}
