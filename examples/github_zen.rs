extern crate hosted_vcs;

use hosted_vcs::github::v3 as github;

fn main() {
    let session = github::anonymous_session().unwrap();
    println!("{}", session.zen().unwrap());
}
