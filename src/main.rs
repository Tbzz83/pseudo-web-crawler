use crate::url_frontier::{Url, UrlFrontier};

pub mod url_frontier;
fn main() {
    let mut frontier = UrlFrontier::new();
    frontier.push_url(Url::new("https://google.com"));
    dbg!(frontier);
}
