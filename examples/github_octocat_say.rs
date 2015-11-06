extern crate hosted_vcs;

use hosted_vcs::github;

fn main() {
    println!("{}", github::api::zen().unwrap());
}
