use crate::url_frontier::url_frontier_soa::{Url, UrlFrontier};

mod url_frontier;

fn main() {
    let mut frontier = UrlFrontier::new();
    for _ in 0..10 {
        frontier.add_url(Url::new("google.com"));
    }
    frontier.add_url(Url::new("yahoo.com"));
}
