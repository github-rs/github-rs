use std::string::ToString;

/// Specifies what to use to sort the results of a [/search/repositories]
/// (https://developer.github.com/v3/search/#search-repositories) query
pub enum SortRepositoriesBy {
    /// Sort by the number of stars
    Stars,
    /// Sort by the number of forks
    Forks,
    /// Sort by the date the repository was last updated
    Updated,
}

/// Specifies what to use to sort the results of a [/search/code]
/// (https://developer.github.com/v3/search/#search-code) query
pub enum SortCodeBy {
    /// Sort by the last time the file was indexed by the GitHub search
    /// infrastructure
    Indexed,
}

/// Specifies what to use to sort the results of a [/search/issues]
/// (https://developer.github.com/v3/search/#search-issues) query
pub enum SortIssuesBy {
    /// Sort by the number of comments
    Comments,
    /// Sort by the date the issue was created
    Created,
    /// Sort by the date the issue was last updated
    Updated,
}

/// Specifies what to use to sort the results of a [/search/users]
/// (https://developer.github.com/v3/search/#search-users) query
pub enum SortUsersBy {
    /// Sort by number of followers
    Followers,
    /// Sort by number of repositories
    Repositories,
    /// Sort by join date
    Joined,
}

/// Specifies how to order query results
pub enum OrderBy {
    /// Sort in ascending order
    Asc,
    /// Sort in descending order (default)
    Desc,
}

impl ToString for SortRepositoriesBy {
    fn to_string(&self) -> String {
        return String::from(match *self {
            SortRepositoriesBy::Stars => "stars",
            SortRepositoriesBy::Forks => "forks",
            SortRepositoriesBy::Updated => "updated",
        });
    }
}

impl ToString for SortCodeBy {
    fn to_string(&self) -> String {
        return String::from(match *self {
            SortCodeBy::Indexed => "indexed",
        });
    }
}

impl ToString for SortIssuesBy {
    fn to_string(&self) -> String {
        return String::from(match *self {
            SortIssuesBy::Comments => "comments",
            SortIssuesBy::Created => "created",
            SortIssuesBy::Updated => "updated",
        });
    }
}

impl ToString for SortUsersBy {
    fn to_string(&self) -> String {
        return String::from(match *self {
            SortUsersBy::Followers => "followers",
            SortUsersBy::Repositories => "repositories",
            SortUsersBy::Joined => "joined",
        });
    }
}

impl ToString for OrderBy {
    fn to_string(&self) -> String {
        return String::from(match *self {
            OrderBy::Asc => "asc",
            OrderBy::Desc => "desc",
        });
    }
}
