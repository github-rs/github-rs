//! Access the Search portion of the GitHub API (https://developer.github.com/v3/search/)
imports!();
use client::GetQueryBuilder;

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
      |?> q -> SearchRepositoriesQ = q_str
    @SearchRepositoriesQ
      |
      |?> sort -> SearchRepositoriesSort = sort_type
    @SearchRepositoriesSort
      |
      |?> order -> SearchRepositoriesOrder = order_type
    @SearchCode
      |
      |?> q -> SearchCodeQ = q_str
    @SearchCodeQ
      |
      |?> sort -> SearchCodeSort = sort_type
    @SearchCodeSort
      |
      |?> order -> SearchCodeOrder = order_type
    @SearchIssues
      |
      |?> q -> SearchIssuesQ = q_str
    @SearchIssuesQ
      |
      |?> sort -> SearchIssuesSort = sort_type
    @SearchIssuesSort
      |
      |?> order -> SearchIssuesOrder = order_type
    @SearchUsers
      |
      |?> q -> SearchUsersQ = q_str
    @SearchUsersQ
      |
      |?> sort -> SearchUsersSort = sort_type
    @SearchUsersSort
      |
      |?> order -> SearchUsersOrder = order_type
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
