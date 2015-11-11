use std::convert::From;
use std::error::Error;
use chrono::{DateTime, UTC};
use rustc_serialize::json;

#[allow(dead_code)]
#[derive(RustcDecodable, Debug)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: String,
    pub size: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub language: String,
    pub has_issues: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: u32,
    pub mirror_url: Option<String>,
    pub open_issues_count: u32,
    pub forks: u32,
    pub open_issues: u32,
    pub watchers: u32,
    pub default_branch: String,
    pub organization: Option<Owner>,
    pub permissions: Option<RepositoryPermissions>,
    pub network_count: u32,
    pub subscribers_count: u32,
}

impl Repository {
    pub fn decode(json_str: &str) -> Result<Repository, Box<Error>> {
        let json_str = json_str.replace("\"type\":", "\"type_\":");
        json::decode(&json_str).map_err(From::from)
    }

    pub fn created_at(&self) -> DateTime<UTC> {
        self.created_at.parse().unwrap()
    }

    pub fn updated_at(&self) -> DateTime<UTC> {
        self.updated_at.parse().unwrap()
    }
}

#[allow(dead_code)]
#[derive(RustcDecodable, Debug)]
pub struct RepositoryPermissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

#[allow(dead_code)]
#[derive(RustcDecodable, Debug)]
pub struct Owner {
    login: String,
    id: u64,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    type_: String,
    site_admin: bool,
}
