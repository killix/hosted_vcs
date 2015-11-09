extern crate hosted_vcs;
extern crate rpassword;

use std::io::{self, Write};
use rpassword::read_password;
use hosted_vcs::github;

fn main() {
    let mut output = io::stdout();
    print!("username: ");
    output.flush().expect("Cannot flush output");

    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Cannot read username");
    username = username.trim().into();

    print!("password: ");
    output.flush().expect("Cannot flush output");
    let password = read_password().expect("Cannot read password");

    let session = github::login_password(username, password).unwrap();
    println!("{:?}", session.current_user().unwrap());
    println!("{}", session.octocat().unwrap());

    println!("Rate Limit:");
    println!("  Limit: {}", session.ratelimit_limit());
    println!("  Remaining: {}", session.ratelimit_remaining());
    println!("  Reset: {}", session.ratelimit_reset());
}
