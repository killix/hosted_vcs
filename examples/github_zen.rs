extern crate hosted_vcs;

use std::env;

use hosted_vcs::github;

fn main() {
    let args = env::args().take(1).collect::<Vec<String>>();

    let say: &str = args.get(0).unwrap_or_else(|| panic!("Missing argument"));
    println!("{}", github::api::octocat_say(say).unwrap());
}
