use std::error::Error;
use std::io::Read;

use hyper::Client;
use hyper::header::{
    self,
    Accept,
    Authorization,
    Basic,
    Bearer,
    ContentType,
    Headers,
    UserAgent
};
use hyper::mime::Mime;
use serde_json;

use super::model::{Github, GithubUser};

const GITHUB_URI: &'static str = "https://api.github.com";

lazy_static! {
    static ref GITHUB_V3: Mime = "application/vnd.github.v3+json".parse().unwrap();
    static ref USER_AGENT: UserAgent = UserAgent("hosted_vcs.rs/0.0.1".into());
}

pub fn password_login(username: String, password: String) -> Result<Session, Box<Error>> {
    let mut headers = Headers::new();

    headers.set(Authorization(
        Basic {
            username: username,
            password: Some(password),
        }
    ));

    headers.set(ContentType::json());
    headers.set(USER_AGENT.clone());
    headers.set(Accept(vec![header::qitem(GITHUB_V3.clone())]));

    let client = Client::new();
    let mut res = try!(client.get(GITHUB_URI).headers(headers.clone()).send());

    let gh: Github = try!(serde_json::from_reader(&mut res));

    Ok(Session {
        client: client,
        headers: headers,
        github: gh,
    })
}

pub fn token_login(token: String) -> Result<Session, Box<Error>> {
    let mut headers = Headers::new();

    headers.set(Authorization(
        Bearer {
            token: token,
        }
    ));

    headers.set(ContentType::json());
    headers.set(USER_AGENT.clone());
    headers.set(Accept(vec![header::qitem(GITHUB_V3.clone())]));

    let client = Client::new();
    let mut res = try!(client.get(GITHUB_URI).headers(headers.clone()).send());

    let gh: Github = try!(serde_json::from_reader(&mut res));

    Ok(Session {
        client: client,
        headers: headers,
        github: gh,
    })
}

pub fn octocat() -> Result<String, Box<Error>> {
    let mut headers = Headers::new();
    headers.set(USER_AGENT.clone());
    headers.set(Accept(vec![header::qitem(GITHUB_V3.clone())]));

    let mut res = try!(Client::new()
        .get(&format!("{}/octocat", GITHUB_URI))
        .headers(headers)
        .send());

    let mut buf = String::new();
    try!(res.read_to_string(&mut buf));

    Ok(buf)
}

pub fn octocat_say(say: &str) -> Result<String, Box<Error>> {
    let mut headers = Headers::new();
    headers.set(USER_AGENT.clone());
    headers.set(Accept(vec![header::qitem(GITHUB_V3.clone())]));

    let mut res = try!(Client::new()
        .get(&format!("{}/octocat?s={}", GITHUB_URI, say))
        .headers(headers)
        .send());

    let mut buf = String::new();
    try!(res.read_to_string(&mut buf));

    Ok(buf)
}

pub fn zen() -> Result<String, Box<Error>> {
    let mut headers = Headers::new();
    headers.set(USER_AGENT.clone());
    headers.set(Accept(vec![header::qitem(GITHUB_V3.clone())]));

    let mut res = try!(Client::new()
        .get(&format!("{}/zen", GITHUB_URI))
        .headers(headers)
        .send());

    let mut buf = String::new();
    try!(res.read_to_string(&mut buf));

    Ok(buf)
}

pub struct Session {
    client: Client,
    headers: Headers,
    github: Github,
}

impl Session {
    pub fn current_user(&self) -> Result<GithubUser, Box<Error>> {
        let mut res = try!(self.client
            .get(&self.github.current_user_url)
            .headers(self.headers.clone())
            .send());

        let user: GithubUser = try!(serde_json::from_reader(&mut res));
        Ok(user)
    }

    pub fn octocat(&self) -> Result<String, Box<Error>> {
        let mut headers = self.headers.clone();
        headers.remove::<ContentType>();
        let mut res = try!(self.client
            .get(&format!("{}/octocat", GITHUB_URI))
            .headers(headers)
            .send());

        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));

        Ok(buf)
    }

    pub fn octocat_say(&self, say: &str) -> Result<String, Box<Error>> {
        let mut headers = self.headers.clone();
        headers.remove::<ContentType>();
        let mut res = try!(self.client
            .get(&format!("{}/octocat?s={}", GITHUB_URI, say))
            .headers(headers)
            .send());

        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));

        Ok(buf)
    }

    pub fn zen(&self) -> Result<String, Box<Error>> {
        let mut headers = self.headers.clone();
        headers.remove::<ContentType>();
        let mut res = try!(self.client
            .get(&format!("{}/zen", GITHUB_URI))
            .headers(headers)
            .send());

        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));

        Ok(buf)
    }
}
