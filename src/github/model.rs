use chrono::{DateTime, UTC};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Github {
    pub current_user_url: String,
    pub current_user_authorizations_html_url: String,
    pub authorizations_url: String,
    pub code_search_url: String,
    pub emails_url: String,
    pub emojis_url: String,
    pub events_url: String,
    pub feeds_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub hub_url: String,
    pub issue_search_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub notifications_url: String,
    pub organization_repositories_url: String,
    pub organization_url: String,
    pub public_gists_url: String,
    pub rate_limit_url: String,
    pub repository_url: String,
    pub repository_search_url: String,
    pub current_user_repositories_url: String,
    pub starred_url: String,
    pub starred_gists_url: String,
    pub team_url: String,
    pub user_url: String,
    pub user_organizations_url: String,
    pub user_repositories_url: String,
    pub user_search_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GithubUser {
  pub login: String,
  pub id: u64,
  pub avatar_url: String,
  pub gravatar_id: String,
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
  #[serde(rename="type")]
  pub type_: String,
  pub site_admin: bool,
  pub name: String,
  pub company: Option<String>,
  pub blog: Option<String>,
  pub location: Option<String>,
  pub email: Option<String>,
  pub hireable: Option<bool>,
  pub bio: Option<String>,
  pub public_repos: u32,
  pub public_gists: u32,
  pub followers: u32,
  pub following: u32,
  created_at: String,
  updated_at: String,
  pub private_gists: Option<u32>,
  pub total_private_repos: Option<u32>,
  pub owned_private_repos: Option<u32>,
  pub disk_usage: Option<u64>,
  pub collaborators: Option<u32>,
  pub plan: Option<GithubUserPlan>,
}

impl GithubUser {
    pub fn created_at(&self) -> DateTime<UTC> {
        self.created_at.parse().unwrap()
    }

    pub fn updated_at(&self) -> DateTime<UTC> {
        self.updated_at.parse().unwrap()
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GithubUserPlan {
    name: String,
    space: u64,
    collaborators: u32,
    private_repos: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GithubOrganization {
    login: String,
    id: u64,
    url: String,
    repos_url: String,
    events_url: String,
    members_url: String,
    public_members_url: String,
    avatar_url: String,
    description: String,
    name: String,
    company: Option<String>,
    blog: String,
    location: Option<String>,
    email: Option<String>,
    public_repos: u32,
    public_gists: u32,
    followers: u32,
    following: u32,
    html_url: String,
    created_at: String,
    updated_at: String,
    #[serde(rename="type")]
    type_: String,
    total_private_repos: Option<u32>,
    owned_private_repos: Option<u32>,
    private_gists: Option<u32>,
    disk_usage: Option<u64>,
    collaborators: Option<u32>,
    billing_email: Option<String>,
    plan: Option<GithubOrganizationPlan>,
}

impl GithubOrganization {
    pub fn created_at(&self) -> DateTime<UTC> {
        self.created_at.parse().unwrap()
    }

    pub fn updated_at(&self) -> DateTime<UTC> {
        self.updated_at.parse().unwrap()
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GithubOrganizationPlan {
    name: String,
    space: u64,
    collaborators: u32,
    private_repos: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GithubRepository {
    id: u64,
    name: String,
    full_name: String,
    owner: GithubUser,
    private: bool,
    html_url: String,
    description: String,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: String,
    size: u32,
    stargazers_count: u32,
    watchers_count: u32,
    language: String,
    has_issues: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    forks_count: u32,
    mirror_url: Option<String>,
    open_issues_count: u32,
    forks: u32,
    open_issues: u32,
    watchers: u32,
    default_branch: String,
    permissions: Option<GithubRepositoryPermissions>,
    network_count: u32,
    subscribers_count: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct GithubRepositoryPermissions {
    admin: bool,
    push: bool,
    pull: bool,
}
