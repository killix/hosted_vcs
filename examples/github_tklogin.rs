extern crate hosted_vcs;
extern crate rpassword;

use std::io::{self, Write};
use rpassword::read_password;
use hosted_vcs::github;

fn main() {
    print!("token: ");
    io::stdout().flush().expect("Cannot flush output");
    let token = read_password().expect("Cannot read token");

    let gh = github::login_token(token).unwrap();
    println!("{:?}", gh.current_user().unwrap());
    println!("{}", gh.octocat().unwrap());
}
