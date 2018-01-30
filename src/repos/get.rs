//! Access the Repos portion of the GitHub API
imports!();
use crate::client::GetQueryBuilder;

new_type!(
    ArchiveReference
    Assignees
    AssigneesUsername
    Branches
    BranchesName
    BranchesNameProtection
    BranchesNameProtectionRestrictions
    BranchesNameProtectionRestrictionsUsers
    BranchesNameProtectionRequiredPullRequestReviews
    BranchesNameProtectionRequiredStatusChecks
    BranchesNameProtectionRequiredStatusChecksContexts
    Collaborators
    CollaboratorsUsername
    CollaboratorsUsernamePermission
    CommitsSha
    CommitsReference
    CommitsComments
    CommitsStatus
    CommitsStatuses
    Commits
    Compare
    CompareBase
    CompareBaseHead
    Contents
    ContentsPath
    ContentsReference
    Contributors
    Events
    Forks
    Issues
    IssuesState
    IssuesComments
    IssuesCommentsId
    IssuesNumber
    IssuesNumberComments
    Languages
    Notifications
    Owner
    Pulls
    PullsComments
    PullsCommentsId
    PullsNumber
    PullsNumberComments
    PullsNumberCommits
    PullsNumberFiles
    PullsNumberRequestedReviewers
    PullsNumberMerge
    Readme
    Repo
    Repos
    Stargazers
    Subscribers
    Subscription
    Tags
    Tarball
    Zipball
);

from!(
    @Assignees
       => AssigneesUsername

    @Branches
       => BranchesName
    @BranchesName
       -> BranchesNameProtection = "protection"
    @BranchesNameProtection
       -> BranchesNameProtectionRestrictions = "restrictions"
       -> BranchesNameProtectionRequiredPullRequestReviews = "required_pull_request_reviews"
       -> BranchesNameProtectionRequiredStatusChecks = "required_status_checks"
    @BranchesNameProtectionRestrictions
       -> BranchesNameProtectionRestrictionsUsers = "users"
    @BranchesNameProtectionRequiredStatusChecks
       -> BranchesNameProtectionRequiredStatusChecksContexts = "contexts"

    @Collaborators
       => CollaboratorsUsername
    @CollaboratorsUsername
       -> CollaboratorsUsernamePermission = "permission"
    @CommitsSha
       -> CommitsComments = "comments"
    @CommitsSha
       -> CommitsStatus = "status"
    @CommitsSha
       -> CommitsStatuses = "statuses"
    @CommitsReference
       -> CommitsComments = "comments"
    @CommitsReference
       -> CommitsStatus = "status"
    @CommitsReference
       -> CommitsStatuses = "statuses"
    @Commits
       => CommitsSha
       => CommitsReference

    @Compare
       => CompareBase
    @CompareBase
       => CompareBaseHead

    @Contents
       => ContentsPath
    @ContentsPath
       ?> ContentsReference = "ref"

    @Issues
       -> IssuesComments = "comments"
    @Issues
       => IssuesNumber
    @IssuesComments
       => IssuesCommentsId
    @IssuesNumber
       -> IssuesNumberComments = "comments"
    @Issues
        ?> IssuesState = "state"

    @Pulls
       -> PullsComments = "comments"
    @PullsComments
       => PullsCommentsId
    @Pulls
       => PullsNumber
    @PullsNumber
       -> PullsNumberComments = "comments"
    @PullsNumber
       -> PullsNumberCommits = "commits"
    @PullsNumber
       -> PullsNumberFiles = "files"
    @PullsNumber
       -> PullsNumberRequestedReviewers = "requested_reviewers"
    @PullsNumber
       -> PullsNumberMerge = "merge"

    @GetQueryBuilder
       -> Repos = "repos"
    @Owner
       => Repo
    @Repo
       -> Assignees = "assignees"
    @Repo
       -> Branches = "branches"
    @Repo
       -> Collaborators = "collaborators"
    @Repo
       -> Commits = "commits"
    @Repo
       -> Compare = "compare"
    @Repo
       -> Contents = "contents"
    @Repo
       -> Contributors = "contributors"
    @Repo
       -> Events = "events"
    @Repo
       -> Forks = "forks"
    @Repo
       -> Languages = "languages"
    @Repo
       -> Notifications = "notifications"
    @Repo
       -> Pulls = "pulls"
       -> Issues = "issues"
    @Repo
       -> Readme = "readme"
    @Repo
       -> Stargazers = "stargazers"
    @Repo
       -> Subscribers = "subscribers"
    @Repo
       -> Subscription = "subscription"
    @Repo
       -> Tags = "tags"
    @Repo
       -> Tarball = "tarball"
       -> Zipball = "zipball"
    @Repos
       => Owner

    @Tarball
       => ArchiveReference
    @Zipball
       => ArchiveReference
);

impl_macro!(
    @Assignees
        |
        |=> username -> AssigneesUsername = username
    @Branches
        |
        |=> name -> BranchesName = name
    @BranchesName
        |=> protection -> BranchesNameProtection
        |
    @BranchesNameProtection
        |=> restrictions -> BranchesNameProtectionRestrictions
        |=> required_pull_request_reviews -> BranchesNameProtectionRequiredPullRequestReviews
        |=> required_status_checks -> BranchesNameProtectionRequiredStatusChecks
        |
    @BranchesNameProtectionRestrictions
        |=> users -> BranchesNameProtectionRestrictionsUsers
        |
    @BranchesNameProtectionRequiredStatusChecks
        |=> contexts -> BranchesNameProtectionRequiredStatusChecksContexts
        |
    @Collaborators
        |
        |=> username -> CollaboratorsUsername = username
    @CollaboratorsUsername
        |=> permission -> CollaboratorsUsernamePermission
        |
    @CommitsSha
        |=> comments -> CommitsComments
        |=> status -> CommitsStatus
        |=> statuses -> CommitsStatuses
        |
    @CommitsReference
        |=> comments -> CommitsComments
        |=> status -> CommitsStatus
        |=> statuses -> CommitsStatuses
        |
    @Commits
        |
        |=> sha -> CommitsSha = sha_str
        |=> reference -> CommitsReference = ref_str
    @Compare
        |
        |=> base -> CompareBase = base_branch
    @CompareBase
        |
        |...> head -> CompareBaseHead = head_branch
    @Contents
        |
        |=> path -> ContentsPath = path_str
    @ContentsPath
        |
        |?> reference -> ContentsReference = ref_str
    @Issues
        |=> comments -> IssuesComments
        |
        |=> number -> IssuesNumber = issue_number
        |?> state -> IssuesState = state
    @IssuesComments
        |
        |=> id -> IssuesCommentsId = comment_id
    @IssuesNumber
        |=> comments -> IssuesNumberComments
        |
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> assignees ->  Assignees
        |=> branches ->  Branches
        |=> collaborators ->  Collaborators
        |=> commits -> Commits
        |=> compare -> Compare
        |=> contents -> Contents
        |=> contributors -> Contributors
        |=> events -> Events
        |=> forks -> Forks
        |=> issues -> Issues
        |=> languages -> Languages
        |=> notifications -> Notifications
        |=> pulls -> Pulls
        |=> readme -> Readme
        |=> stargazers -> Stargazers
        |=> subscribers -> Subscribers
        |=> subscription -> Subscription
        |=> tags -> Tags
        |=> tarball -> Tarball
        |=> zipball -> Zipball
        |
    @Repos
        |
        |=> owner ->  Owner = username_str
    @Pulls
        |=> comments -> PullsComments
        |
    @Pulls
        |
        |=> number -> PullsNumber = number_str
    @PullsComments
        |
        |=> id -> PullsCommentsId = id_str
    @PullsNumber
        |=> comments -> PullsNumberComments
        |=> commits -> PullsNumberCommits
        |=> files -> PullsNumberFiles
        |=> requested_reviewers -> PullsNumberRequestedReviewers
        |=> merge -> PullsNumberMerge
        |
    @Tarball
        |
        |=> reference -> ArchiveReference = ref_str
    @Zipball
        |
        |=> reference -> ArchiveReference = ref_str
);

exec!(ArchiveReference);
exec!(Assignees);
exec!(AssigneesUsername);
exec!(Branches);
exec!(BranchesName);
exec!(BranchesNameProtection);
exec!(BranchesNameProtectionRestrictions);
exec!(BranchesNameProtectionRestrictionsUsers);
exec!(BranchesNameProtectionRequiredPullRequestReviews);
exec!(BranchesNameProtectionRequiredStatusChecks);
exec!(BranchesNameProtectionRequiredStatusChecksContexts);
exec!(Collaborators);
exec!(CollaboratorsUsername);
exec!(CollaboratorsUsernamePermission);
exec!(Commits);
exec!(CommitsSha);
exec!(CommitsReference);
exec!(CommitsComments);
exec!(CommitsStatus);
exec!(CommitsStatuses);
exec!(CompareBaseHead);
exec!(ContentsPath);
exec!(ContentsReference);
exec!(Contributors);
exec!(Events);
exec!(Forks);
exec!(Issues);
exec!(IssuesState);
exec!(IssuesComments);
exec!(IssuesCommentsId);
exec!(IssuesNumber);
exec!(IssuesNumberComments);
exec!(Languages);
exec!(Notifications);
exec!(Pulls);
exec!(PullsComments);
exec!(PullsCommentsId);
exec!(PullsNumber);
exec!(PullsNumberComments);
exec!(PullsNumberCommits);
exec!(PullsNumberFiles);
exec!(PullsNumberRequestedReviewers);
exec!(PullsNumberMerge);
exec!(Readme);
exec!(Repo);
exec!(Stargazers);
exec!(Subscribers);
exec!(Subscription);
exec!(Tags);
exec!(Tarball);
exec!(Zipball);
