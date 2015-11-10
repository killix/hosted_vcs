use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GithubError {
    description: String,
}

impl GithubError {
    pub fn new(description: &str) -> Self {
        GithubError {
            description: description.into(),
        }
    }
}

impl Error for GithubError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description)
    }
}
