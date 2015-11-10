use std::error::Error;
use serde_json;

use super::super::model::{User, Organization, Repository};
use super::*;

impl Session<Authenticated> {
    pub fn current_user(&self) -> Result<User, Box<Error>> {
        let request = self.client.get(&self.github.current_user_url);
        let mut response = try!(self.send_request(request, None));

        let user: User = try!(serde_json::from_reader(&mut response));
        Ok(user)
    }

    pub fn get_user(&self, user: &str) -> Result<User, Box<Error>> {
        let url = self.github.user_url.replace("{user}", user);
        let request = self.client.get(&url);
        let mut response = try!(self.send_request(request, None));

        let user: User = try!(serde_json::from_reader(&mut response));
        Ok(user)
    }

    pub fn get_organization(&self, org: &str) -> Result<Organization, Box<Error>> {
        let url = self.github.organization_url.replace("{org}", org);
        let request = self.client.get(&url);
        let mut response = try!(self.send_request(request, None));

        let org: Organization = try!(serde_json::from_reader(&mut response));
        Ok(org)
    }

    pub fn get_repository(&self, owner: &str, repo: &str) -> Result<Repository, Box<Error>> {
        let url = self.github.repository_url.replace("{owner}", owner).replace("{repo}", repo);
        let request = self.client.get(&url);
        let mut response = try!(self.send_request(request, None));

        let repo: Repository = try!(serde_json::from_reader(&mut response));
        Ok(repo)
    }
}
