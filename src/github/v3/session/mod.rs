use std::any::Any;
use std::cell::Cell;
use std::error::Error;
use std::io::Read;
use std::marker::PhantomData;

use hyper::Client;
use hyper::client::{RequestBuilder, Response, IntoUrl};
use hyper::header::{
    self,
    Accept,
    Authorization,
    Basic,
    Bearer,
    ContentType,
    ETag,
    EntityTag,
    Headers,
    UserAgent
};
use hyper::mime::Mime;
use rustc_serialize::json;

use super::error::GithubError;
use super::model::Root;

mod authenticated;

const GITHUB_URI: &'static str = "https://api.github.com";

lazy_static! {
    static ref GITHUB_V3: Mime = "application/vnd.github.v3+json".parse().unwrap();
    static ref USER_AGENT: UserAgent = UserAgent("hosted_vcs.rs/0.0.1".into());
}

pub fn anonymous() -> Result<Session<Anonymous>, Box<Error>> {
    let mut headers = Headers::new();

    headers.set(ContentType::json());
    headers.set(USER_AGENT.clone());

    let accept = vec![header::qitem(GITHUB_V3.clone())];
    headers.set(Accept(accept));

    let client = Client::new();
    let mut res = try!(client.get(GITHUB_URI).headers(headers.clone()).send());

    let mut buf = String::new();
    try!(res.read_to_string(&mut buf));
    let root: Root = try!(json::decode(&buf));
    
    let ratelimit = try!(RateLimit::parse(&res.headers));

    Ok(Session::<Anonymous> {
        client: client,
        headers: headers,
        github: root,
        ratelimit: Cell::new(ratelimit),
        _kind: PhantomData,
    })
}

fn login(mut headers: Headers) -> Result<Session<Authenticated>, Box<Error>> {
    headers.set(ContentType::json());
    headers.set(USER_AGENT.clone());

    let accept = vec![header::qitem(GITHUB_V3.clone())];
    headers.set(Accept(accept));

    let client = Client::new();
    let mut res = try!(client.get(GITHUB_URI).headers(headers.clone()).send());

    let mut buf = String::new();
    try!(res.read_to_string(&mut buf));
    let root: Root = try!(json::decode(&buf));

    let ratelimit = try!(RateLimit::parse(&res.headers));

    Ok(Session {
        client: client,
        headers: headers,
        github: root,
        ratelimit: Cell::new(ratelimit),
        _kind: PhantomData,
    })
}

pub fn login_password(username: String, password: String) -> Result<Session<Authenticated>, Box<Error>> {
    let mut headers = Headers::new();

    headers.set(Authorization(
        Basic {
            username: username,
            password: Some(password),
        }
    ));

    login(headers)
}

pub fn login_token(token: String) -> Result<Session<Authenticated>, Box<Error>> {
    let mut headers = Headers::new();

    headers.set(Authorization(
        Bearer {
            token: token,
        }
    ));

    login(headers)
}

pub struct Anonymous;
pub struct Authenticated;

pub struct Session<Kind: Any> {
    client: Client,
    headers: Headers,
    github: Root,
    ratelimit: Cell<RateLimit>,
    _kind: PhantomData<Kind>,
}

impl<Kind: Any> Session<Kind> {
    fn send_request<T: IntoUrl>(&self, request: RequestBuilder<T>, etag: Option<String>) -> Result<Response, Box<Error>> {
        let mut headers = self.headers.clone();
        if let Some(etag) = etag {
            headers.set(ETag(EntityTag::strong(etag)));
        }

        let response = try!(request.headers(headers).send());
        let ratelimit = try!(RateLimit::parse(&response.headers));
        self.ratelimit.set(ratelimit);

        Ok(response)
    }

    pub fn ratelimit_limit(&self) -> u32 {
        self.ratelimit.get().limit
    }

    pub fn ratelimit_remaining(&self) -> u32 {
        self.ratelimit.get().remaining
    }

    pub fn ratelimit_reset(&self) -> u64 {
        self.ratelimit.get().reset
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

#[derive(Copy, Clone)]
struct RateLimit {
    limit: u32,
    remaining: u32,
    reset: u64,
}

impl RateLimit {
    fn parse(headers: &Headers) -> Result<RateLimit, Box<Error>> {
        let ratelimit_limit = match headers.get_raw("X-RateLimit-Limit") {
            Some(value) => value,
            None => return Err(Box::new(GithubError::new("Missing header 'X-RateLimit-Limit'"))),
        };

        let ratelimit_limit = try!(String::from_utf8(ratelimit_limit[0].clone()));
        let ratelimit_limit = try!(ratelimit_limit.parse());

        let ratelimit_remaining = match headers.get_raw("X-RateLimit-Remaining") {
            Some(value) => value,
            None => return Err(Box::new(GithubError::new("Missing header 'X-RateLimit-Remaining'"))),
        };

        let ratelimit_remaining = try!(String::from_utf8(ratelimit_remaining[0].clone()));
        let ratelimit_remaining = try!(ratelimit_remaining.parse());

        let ratelimit_reset = match headers.get_raw("X-RateLimit-Reset") {
            Some(value) => value,
            None => return Err(Box::new(GithubError::new("Missing header 'X-RateLimit-reset'"))),
        };

        let ratelimit_reset = try!(String::from_utf8(ratelimit_reset[0].clone()));
        let ratelimit_reset = try!(ratelimit_reset.parse());

        Ok(RateLimit {
            limit: ratelimit_limit,
            remaining: ratelimit_remaining,
            reset: ratelimit_reset,
        })
    }
}
