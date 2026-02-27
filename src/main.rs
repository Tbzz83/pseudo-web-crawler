use crate::url_frontier::url_frontier_soa::{Url, UrlFrontier};

mod url_frontier;

fn main() {
    let mut frontier = UrlFrontier::new();
    for _ in 0..1000000 {
        frontier.push_url(Url::new("https://google.com"));
    }
}
