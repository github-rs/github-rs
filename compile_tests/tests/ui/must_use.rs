extern crate github_rs;

use github_rs::client::Github;

#[warn(unused_must_use)] // compiletest allows `unused` for UI tests, so we restore the warning here
fn main() {
    let g = Github::new("").unwrap();
    g.get().user().starred().owner("");
    g.get().gists();
}

