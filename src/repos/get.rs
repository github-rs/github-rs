//! Access the Repos portion of the GitHub API
imports!();
use client::GetQueryBuilder;

new_type!(
    Assignees
    Branches
    Collaborators
    CollaboratorsUsername
    CollaboratorsUsernamePermission
    CommitsSha
    CommitsRef
    CommitsComments
    CommitsStatus
    CommitsStatuses
    Commits
    Contents
    ContentsPath
    ContentsReference
    Contributors
    Events
    Forks
    Issues
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
);

from!(
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
    @CommitsRef
       -> CommitsComments = "comments"
    @CommitsRef
       -> CommitsStatus = "status"
    @CommitsRef
       -> CommitsStatuses = "statuses"
    @Commits
       => CommitsSha
       => CommitsRef

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
    @Repos
       => Owner

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
);

impl_macro!(
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
    @CommitsRef
        |=> comments -> CommitsComments
        |=> status -> CommitsStatus
        |=> statuses -> CommitsStatuses
        |
    @Commits
        |
        |=> sha -> CommitsSha = sha_str
        |=> reference -> CommitsRef = ref_str
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
);

exec!(Assignees);
exec!(Branches);
exec!(Collaborators);
exec!(CollaboratorsUsername);
exec!(CollaboratorsUsernamePermission);
exec!(Commits);
exec!(CommitsSha);
exec!(CommitsRef);
exec!(CommitsComments);
exec!(CommitsStatus);
exec!(CommitsStatuses);
exec!(ContentsPath);
exec!(ContentsReference);
exec!(Contributors);
exec!(Events);
exec!(Forks);
exec!(Issues);
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
