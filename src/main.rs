use crate::url_frontier::url_frontier_soa::{Url, UrlFrontier};

mod url_frontier;
mod constants;

fn main() {
    let mut frontier = UrlFrontier::new();
    frontier.add_url(Url::new("google.com"));
    frontier.add_url(Url::new("yahoo.com"));
    //dbg!(frontier);
}
