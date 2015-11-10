extern crate hosted_vcs;
extern crate rpassword;

use std::io::{self, Write};
use rpassword::read_password;
use hosted_vcs::github::v3 as github;

fn main() {
    print!("token: ");
    io::stdout().flush().expect("Cannot flush output");
    let token = read_password().expect("Cannot read token");

    let session = github::login_token(token).unwrap();
    println!("{:?}", session.current_user().unwrap());
    println!("{}", session.octocat().unwrap());

    println!("Rate Limit:");
    println!("  Limit: {}", session.ratelimit_limit());
    println!("  Remaining: {}", session.ratelimit_remaining());
    println!("  Reset: {}", session.ratelimit_reset());
}
