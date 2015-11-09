extern crate hosted_vcs;

use std::env;
use hosted_vcs::github;

fn main() {
    let session = github::anonymous_session().unwrap();

    match env::args().nth(1) {
        Some(say) => println!("{}", session.octocat_say(&say).unwrap()),
        None => println!("{}", session.octocat().unwrap()),
    }
}
