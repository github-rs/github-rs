//! Access the Search portion of the GitHub API (https://developer.github.com/v3/search/)
imports!();
use client::GetQueryBuilder;
use search::types::{SortRepositoriesBy, SortCodeBy, SortIssuesBy, SortUsersBy, OrderBy};

new_type!(
    Search
    SearchRepositories
    SearchRepositoriesQ
    SearchRepositoriesSort
    SearchRepositoriesOrder
    SearchCode
    SearchCodeQ
    SearchCodeSort
    SearchCodeOrder
    SearchIssues
    SearchIssuesQ
    SearchIssuesSort
    SearchIssuesOrder
    SearchUsers
    SearchUsersQ
    SearchUsersSort
    SearchUsersOrder
);

from!(
    @GetQueryBuilder
      -> Search = "search"
    @Search
      -> SearchRepositories = "repositories"
      -> SearchCode = "code"
      -> SearchIssues = "issues"
      -> SearchUsers = "users"
    @SearchRepositories
      ?> SearchRepositoriesQ = "q"
    @SearchRepositoriesQ
      ?> SearchRepositoriesSort = "sort"
    @SearchRepositoriesSort
      ?> SearchRepositoriesOrder = "order"
    @SearchCode
      ?> SearchCodeQ = "q"
    @SearchCodeQ
      ?> SearchCodeSort = "sort"
    @SearchCodeSort
      ?> SearchCodeOrder = "order"
    @SearchUsers
      ?> SearchUsersQ = "q"
    @SearchUsersQ
      ?> SearchUsersSort = "sort"
    @SearchUsersSort
      ?> SearchUsersOrder = "order"
    @SearchIssues
      ?> SearchIssuesQ = "q"
    @SearchIssuesQ
      ?> SearchIssuesSort = "sort"
    @SearchIssuesSort
      ?> SearchIssuesOrder = "order"
);

impl_macro!(
    @Search
      |=> repositories -> SearchRepositories
      |=> code -> SearchCode
      |=> issues -> SearchIssues
      |=> users -> SearchUsers
      |
    @SearchRepositories
      |
      |?> q -> SearchRepositoriesQ = q_str : &str
    @SearchRepositoriesQ
      |
      |?> sort -> SearchRepositoriesSort = sort_type : SortRepositoriesBy
    @SearchRepositoriesSort
      |
      |?> order -> SearchRepositoriesOrder = order_type : OrderBy
    @SearchCode
      |
      |?> q -> SearchCodeQ = q_str : &str
    @SearchCodeQ
      |
      |?> sort -> SearchCodeSort = sort_type : SortCodeBy
    @SearchCodeSort
      |
      |?> order -> SearchCodeOrder = order_type : OrderBy
    @SearchIssues
      |
      |?> q -> SearchIssuesQ = q_str : &str
    @SearchIssuesQ
      |
      |?> sort -> SearchIssuesSort = sort_type : SortIssuesBy
    @SearchIssuesSort
      |
      |?> order -> SearchIssuesOrder = order_type : OrderBy
    @SearchUsers
      |
      |?> q -> SearchUsersQ = q_str : &str
    @SearchUsersQ
      |
      |?> sort -> SearchUsersSort = sort_type : SortUsersBy
    @SearchUsersSort
      |
      |?> order -> SearchUsersOrder = order_type : OrderBy
);

exec!(SearchRepositoriesQ);
exec!(SearchRepositoriesSort);
exec!(SearchRepositoriesOrder);
exec!(SearchCodeQ);
exec!(SearchCodeSort);
exec!(SearchCodeOrder);
exec!(SearchIssuesQ);
exec!(SearchIssuesSort);
exec!(SearchIssuesOrder);
exec!(SearchUsersQ);
exec!(SearchUsersSort);
exec!(SearchUsersOrder);
