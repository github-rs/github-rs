//! Trait defition related to the User on Github
use hyper::status::StatusCode;
use json::{SSHKey, Email, User, PatchUser};
use requests::*;
use github::Client;
use error::*;
use serde_json;

/// Trait used to define access to endpoints grouped under `Users` in the Github API
/// specification
pub trait Users {
    // Finished -- for now
    // User
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user
    /// ### Description
    /// Returns a `User` Struct for the authenticated user.
    fn get_user(&self) -> Result<User>;

    /// ### Request Type:
    /// `PATCH`
    /// ### Endpoint:
    /// /user
    /// ### Description
    /// Returns a `User` struct of the authenticated user once their data has been updated
    fn patch_user(&self, user: PatchUser) -> Result<User>;

    // Users
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/keys
    /// ### Description
    /// Returns a Vector of all `SSHKeys` from the username specified
    fn get_users_username(&self, username: &str) -> Result<User>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users
    /// ### Description
    /// Returns a vector of `User`s from the website. This is a paginated request to the endpoint.
    fn get_users(&self, user_id: Option<u64>, per_page: Option<u64>) -> Result<Vec<User>>;

    // Followers User/Users
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/followers
    /// ### Description
    /// Returns a vector of `User`s followin the authenticated user. This is a paginated request
    /// to the endpoint.
    fn get_user_followers(&self,
                          page_num: Option<u64>,
                          per_page: Option<u64>)
                          -> Result<Vec<User>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/followers
    /// ### Description
    /// Returns a vector of `User`s following the username passed into the function. This is a
    /// paginated request to the endpoint.
    fn get_users_username_followers(&self,
                                    username: &str,
                                    page_num: Option<u64>,
                                    per_page: Option<u64>)
                                    -> Result<Vec<User>>;

    // Email
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/emails
    /// ### Description
    /// Returns a vector of `Email`s of the authenticated user. This is a paginated request to the
    /// endpoint.
    fn get_user_emails(&self, page_num: Option<u64>, per_page: Option<u64>) -> Result<Vec<Email>>;

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /user/emails
    /// ### Description
    /// Returns a vector of `Email`s when they've been added to the user's profile
    fn post_user_emails(&self, emails: Vec<String>) -> Result<Vec<Email>>;

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /user/emails
    /// ### Description
    /// Returns a boolean value. True if the email(s) were deleted and false otherwise. The email
    /// strings must match verbatim for the corresponding email to be deleted.
    fn delete_user_emails(&self, emails: Vec<String>) -> Result<bool>;

    // User Following
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/following
    /// ### Description
    /// Returns a vector of all `User`s the authenticated user is following. This is a paginated
    /// request to the endpoint.
    fn get_user_following(&self,
                          page_num: Option<u64>,
                          per_page: Option<u64>)
                          -> Result<Vec<User>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/following/:username
    /// ### Description
    /// Returns true if the authenticated user is following the username passed to the function and
    /// false otherwise.
    fn get_user_following_username(&self, username: &str) -> Result<bool>;

    /// ### Request Type:
    /// `PUT`
    /// ### Endpoint:
    /// /user/following/:username
    /// ### Description
    /// Returns true is the user is now following the given username and false otherwise
    fn put_user_following_username(&self, username: &str) -> Result<bool>;

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /user/following/:username
    /// ### Description
    /// Returns true if the authenticated user unfollowed the username passed in and false
    /// otherwise.
    fn delete_user_following_username(&self, username: &str) -> Result<bool>;

    // Users Following
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/following
    /// ### Description
    /// Returns a vector of `User`s that the given username is following. This is a paginated
    /// request to the endpoint.
    fn get_users_username_following(&self,
                                    username: &str,
                                    page_num: Option<u64>,
                                    per_page: Option<u64>)
                                    -> Result<Vec<User>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/following/:target_user
    /// ### Description
    /// Returns true is the username is following the target user and false otherwise.
    fn get_users_username_following_target_user(&self,
                                                username: &str,
                                                target_user: &str)
                                                -> Result<bool>;

    // Keys User
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/keys
    /// ### Description
    /// Returns a Vector of all `SSHKey`s that the authenticate user might have.
    fn get_user_keys(&self) -> Result<Vec<SSHKey>>;

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/keys/:id
    /// ### Description
    /// Returns an `SSHKey` to the user that is specified by id. If it does not exist a None value
    /// is returned.
    fn get_user_keys_id(&self, id: u64) -> Result<Option<SSHKey>>;

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /user/keys
    /// ### Description
    /// Creates a new SSH key sending it back if it worked out properly.
    fn post_user_keys(&self, new_key: SSHKey) -> Result<SSHKey>;

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /user/keys
    /// ### Description
    /// Returns a boolean value. True is the ssh key was deleted and false otherwise. The id number
    /// of the key is what is used to determine which one to delete.
    fn delete_user_keys(&self, id: u64) -> Result<bool>;

    // Keys Users
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/keys
    /// ### Description
    /// Returns a Vector of all `SSHKeys` from the username specified
    fn get_users_username_keys(&self, username: &str) -> Result<Vec<SSHKey>>;

    // Untestable for myself but will make on best guess anyways
    // fn put_users_username_site_admin(&self, username: &str) -> bool;
    // fn delete_users_username_site_admin(&self, username: &str) -> bool;
    // fn put_users_username_suspended(&self, username: &str) -> bool;
    // fn delete_users_username_suspended(&self, username: &str) -> bool;

    // Experimental subject to change in API. Don't Implement for now
    // fn get_user_gpg_keys(&self) -> Vec<GPGKey>
    // fn get_user_gpg_keys_id(&self, id: u64) -> GPGKey
    // fn post_user_gpg_keys(&self, key: GPGKeyPost) -> GPGKey
    // fn delete_user_gpg_keys_id(&self, id: u64) -> bool
}

// Doc comments are duplicated so that regardless of trait or client they see how to use it
impl Users for Client {
    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user
    /// ### Description
    /// Returns a `User` Struct for the authenticated user.
    fn get_user(&self) -> Result<User> {
        let url = "https://api.github.com/user";
        let data = get(url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))

    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username
    /// ### Description
    /// Returns a `User` Struct for the person whose name is passed into the function
    fn get_users_username(&self, username: &str) -> Result<User> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users
    /// ### Description
    /// Returns a vector of `User`s from the website. This is a paginated request to the endpoint.
    fn get_users(&self, user_id: Option<u64>, per_page: Option<u64>) -> Result<Vec<User>> {
        let mut url = String::from("https://api.github.com/users");
        if let Some(user_id) = user_id {
            url.push_str("?since=");
            url.push_str(&user_id.to_string());
        }
        if let Some(num) = per_page {
            url.push_str("&per_page=");
            url.push_str(&num.to_string());
        }

        let data = get(&url, self.get_headers().clone())?;

        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/emails
    /// ### Description
    /// Returns a vector of `Email`s of the authenticated user. This is a paginated request to the
    /// endpoint.
    fn get_user_emails(&self, page_num: Option<u64>, per_page: Option<u64>) -> Result<Vec<Email>> {
        let mut url = String::from("https://api.github.com/user/emails");
        paginate!(url,page_num,per_page);
        let data = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/followers
    /// ### Description
    /// Returns a vector of `User`s following the username passed into the function. This is a
    /// paginated request to the endpoint.
    fn get_users_username_followers(&self,
                                    username: &str,
                                    page_num: Option<u64>,
                                    per_page: Option<u64>)
                                    -> Result<Vec<User>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/followers");
        paginate!(url,page_num,per_page);
        let data = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/followers
    /// ### Description
    /// Returns a vector of `User`s followin the authenticated user. This is a paginated request
    /// to the endpoint.
    fn get_user_followers(&self,
                          page_num: Option<u64>,
                          per_page: Option<u64>)
                          -> Result<Vec<User>> {
        let mut url = String::from("https://api.github.com/user/followers");
        paginate!(url,page_num,per_page);
        let data = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))

    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/following
    /// ### Description
    /// Returns a vector of `User`s that the given username is following. This is a paginated
    /// request to the endpoint.
    fn get_users_username_following(&self,
                                    username: &str,
                                    page_num: Option<u64>,
                                    per_page: Option<u64>)
                                    -> Result<Vec<User>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/following");
        paginate!(url,page_num,per_page);
        let data = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))

    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/following
    /// ### Description
    /// Returns a vector of all `User`s the authenticated user is following. This is a paginated
    /// request to the endpoint.
    fn get_user_following(&self,
                          page_num: Option<u64>,
                          per_page: Option<u64>)
                          -> Result<Vec<User>> {
        let mut url = String::from("https://api.github.com/user/following");
        paginate!(url,page_num,per_page);
        let data = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&data))

    }

    /// ### Request Type:
    /// `PATCH`
    /// ### Endpoint:
    /// /user
    /// ### Description
    /// Returns a `User` struct of the authenticated user once their data has been updated
    fn patch_user(&self, user: PatchUser) -> Result<User> {
        let url = "https://api.github.com/user";
        let res = patch(url,
                        self.get_headers().clone(),
                        serde_json::to_string(&user)?)?;
        try_serde!(serde_json::from_str(&res))
    }

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /user/emails
    /// ### Description
    /// Returns a vector of `Email`s when they've been added to the user's profile
    fn post_user_emails(&self, emails: Vec<String>) -> Result<Vec<Email>> {
        let url = "https://api.github.com/user/emails";
        let res = post(url,
                       self.get_headers().clone(),
                       serde_json::to_string(&emails)?)?;
        try_serde!(serde_json::from_str(&res))
    }

    /// ### Request Type:
    /// `PUT`
    /// ### Endpoint:
    /// /user/following/:username
    /// ### Description
    /// Returns true is the user is now following the given username and false otherwise
    fn put_user_following_username(&self, username: &str) -> Result<bool> {
        let mut url = String::from("https://api.github.com/user/following/");
        url.push_str(username);
        let res = put(&url, self.get_headers().clone())?;
        Ok(res == StatusCode::NoContent)
    }

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /user/following/:username
    /// ### Description
    /// Returns true if the authenticated user unfollowed the username passed in and false
    /// otherwise.
    fn delete_user_following_username(&self, username: &str) -> Result<bool> {
        let mut url = String::from("https://api.github.com/user/following/");
        url.push_str(username);
        let res = delete(&url, self.get_headers().clone())?;
        Ok(res == StatusCode::NoContent)
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/following/:username
    /// ### Description
    /// Returns true if the authenticated user is following the username passed to the function and
    /// false otherwise.
    fn get_user_following_username(&self, username: &str) -> Result<bool> {
        let mut url = String::from("https://api.github.com/user/following/");
        url.push_str(username);
        let res = get_status_code(&url, self.get_headers().clone())?;
        Ok(res == StatusCode::NoContent)
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/following/:target_user
    /// ### Description
    /// Returns true is the username is following the target user and false otherwise.
    fn get_users_username_following_target_user(&self,
                                                username: &str,
                                                target_user: &str)
                                                -> Result<bool> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/following/");
        url.push_str(target_user);
        let res = get_status_code(&url, self.get_headers().clone())?;
        Ok(res == StatusCode::NoContent)
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /users/:username/keys
    /// ### Description
    /// Returns a Vector of all `SSHKeys` from the username specified
    fn get_users_username_keys(&self, username: &str) -> Result<Vec<SSHKey>> {
        let mut url = String::from("https://api.github.com/users/");
        url.push_str(username);
        url.push_str("/keys");
        let res = get(&url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&res))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/keys
    /// ### Description
    /// Returns a Vector of all `SSHKey`s that the authenticate user might have.
    fn get_user_keys(&self) -> Result<Vec<SSHKey>> {
        let url = "https://api.github.com/user/keys";
        let res = get(url, self.get_headers().clone())?;
        try_serde!(serde_json::from_str(&res))
    }

    /// ### Request Type:
    /// `GET`
    /// ### Endpoint:
    /// /user/keys/:id
    /// ### Description
    /// Returns an `SSHKey` to the user that is specified by id. If it does not exist a None value
    /// is returned.
    fn get_user_keys_id(&self, id: u64) -> Result<Option<SSHKey>> {
        let mut url = String::from("https://api.github.com/user/keys/");
        url.push_str(&id.to_string());
        let res = get(&url, self.get_headers().clone())?;

        // Try and fix this
        if let Ok(serial) = serde_json::from_str(&res) {
            Ok(Some(serial))
        } else {
            Ok(None)
        }
    }

    /// ### Request Type:
    /// `POST`
    /// ### Endpoint:
    /// /user/keys
    /// ### Description
    /// Creates a new SSH key sending it back if it worked out properly.
    fn post_user_keys(&self, new_key: SSHKey) -> Result<SSHKey> {
        let url = "https://api.github.com/user/keys";
        let res = post(url,
                       self.get_headers().clone(),
                       serde_json::to_string(&new_key)?)?;
        try_serde!(serde_json::from_str(&res))

    }

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /user/keys
    /// ### Description
    /// Returns a boolean value. True is the ssh key was deleted and false otherwise. The id number
    /// of the key is what is used to determine which one to delete.
    fn delete_user_keys(&self, id: u64) -> Result<bool> {
        let mut url = String::from("https://api.github.com/user/keys");
        url.push_str(&id.to_string());
        let res = delete(&url, self.get_headers().clone())?;
        Ok(res == StatusCode::NoContent)
    }

    /// ### Request Type:
    /// `DELETE`
    /// ### Endpoint:
    /// /user/emails
    /// ### Description
    /// Returns a boolean value. True if the email(s) were deleted and false otherwise. The email
    /// strings must match verbatim for the corresponding email to be deleted.
    fn delete_user_emails(&self, emails: Vec<String>) -> Result<bool> {
        let url = "https://api.github.com/user/emails";
        let res = delete_with_data(url,
                                   self.get_headers().clone(),
                                   serde_json::to_string(&emails)? )?;
        Ok(res == StatusCode::NoContent)
    }
}
