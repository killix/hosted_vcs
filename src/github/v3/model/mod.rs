mod user;
mod organization;
mod repository;

pub use self::user::{User, UserPlan};
pub use self::organization::{Organization, OrganizationPlan};
pub use self::repository::{Repository, RepositoryPermissions};

// {
//   "current_user_url": "https://api.github.com/user",
//   "current_user_authorizations_html_url": "https://github.com/settings/connections/applications{/client_id}",
//   "authorizations_url": "https://api.github.com/authorizations",
//   "code_search_url": "https://api.github.com/search/code?q={query}{&page,per_page,sort,order}",
//   "emails_url": "https://api.github.com/user/emails",
//   "emojis_url": "https://api.github.com/emojis",
//   "events_url": "https://api.github.com/events",
//   "feeds_url": "https://api.github.com/feeds",
//   "followers_url": "https://api.github.com/user/followers",
//   "following_url": "https://api.github.com/user/following{/target}",
//   "gists_url": "https://api.github.com/gists{/gist_id}",
//   "hub_url": "https://api.github.com/hub",
//   "issue_search_url": "https://api.github.com/search/issues?q={query}{&page,per_page,sort,order}",
//   "issues_url": "https://api.github.com/issues",
//   "keys_url": "https://api.github.com/user/keys",
//   "notifications_url": "https://api.github.com/notifications",
//   "organization_repositories_url": "https://api.github.com/orgs/{org}/repos{?type,page,per_page,sort}",
//   "organization_url": "https://api.github.com/orgs/{org}",
//   "public_gists_url": "https://api.github.com/gists/public",
//   "rate_limit_url": "https://api.github.com/rate_limit",
//   "repository_url": "https://api.github.com/repos/{owner}/{repo}",
//   "repository_search_url": "https://api.github.com/search/repositories?q={query}{&page,per_page,sort,order}",
//   "current_user_repositories_url": "https://api.github.com/user/repos{?type,page,per_page,sort}",
//   "starred_url": "https://api.github.com/user/starred{/owner}{/repo}",
//   "starred_gists_url": "https://api.github.com/gists/starred",
//   "team_url": "https://api.github.com/teams",
//   "user_url": "https://api.github.com/users/{user}",
//   "user_organizations_url": "https://api.github.com/user/orgs",
//   "user_repositories_url": "https://api.github.com/users/{user}/repos{?type,page,per_page,sort}",
//   "user_search_url": "https://api.github.com/search/users?q={query}{&page,per_page,sort,order}"
// }

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Root {
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
