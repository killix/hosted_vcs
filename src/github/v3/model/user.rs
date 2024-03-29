use chrono::{DateTime, UTC};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct User {
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
  pub plan: Option<UserPlan>,
}

impl User {
    pub fn created_at(&self) -> DateTime<UTC> {
        self.created_at.parse().unwrap()
    }

    pub fn updated_at(&self) -> DateTime<UTC> {
        self.updated_at.parse().unwrap()
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct UserPlan {
    name: String,
    space: u64,
    collaborators: u32,
    private_repos: u32,
}
