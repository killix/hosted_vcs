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
  pub private_gists: u32,
  pub total_private_repos: u32,
  pub owned_private_repos: u32,
  pub disk_usage: u64,
  pub collaborators: u32,
  pub plan: GithubPlan,
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
pub struct GithubPlan {
    name: String,
    space: u64,
    collaborators: u32,
    private_repos: u32,
}
