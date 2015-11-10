use chrono::{DateTime, UTC};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Organization {
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
    plan: Option<OrganizationPlan>,
}

impl Organization {
    pub fn created_at(&self) -> DateTime<UTC> {
        self.created_at.parse().unwrap()
    }

    pub fn updated_at(&self) -> DateTime<UTC> {
        self.updated_at.parse().unwrap()
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OrganizationPlan {
    name: String,
    space: u64,
    collaborators: u32,
    private_repos: u32,
}
