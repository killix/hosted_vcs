use std::convert::From;
use std::error::Error;
use chrono::{DateTime, UTC};
use rustc_serialize::json;

#[allow(dead_code)]
#[derive(RustcDecodable, Debug)]
pub struct Organization {
    pub login: String,
    pub id: u64,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: String,
    pub name: String,
    pub company: Option<String>,
    pub blog: String,
    pub location: Option<String>,
    pub email: Option<String>,
    pub public_repos: u32,
    pub public_gists: u32,
    pub followers: u32,
    pub following: u32,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub type_: String,
    pub total_private_repos: Option<u32>,
    pub owned_private_repos: Option<u32>,
    pub private_gists: Option<u32>,
    pub disk_usage: Option<u64>,
    pub collaborators: Option<u32>,
    pub billing_email: Option<String>,
    pub plan: Option<OrganizationPlan>,
}

impl Organization {
    pub fn decode(json_str: &str) -> Result<Organization, Box<Error>> {
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
pub struct OrganizationPlan {
    pub name: String,
    pub space: u64,
    pub collaborators: u32,
    pub private_repos: u32,
}
