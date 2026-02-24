use crate::url_frontier_oop::{Url as UrlOop, UrlFrontier as UrlFrontierOop};
pub mod url_frontier_aos;
pub mod url_frontier_oop;
pub mod url_frontier_soa; // SOA is not yet implemented

fn main() {
    let mut frontier = UrlFrontierOop::new();
    for _ in 0..1000000 {
        frontier.push_url(UrlOop::new("https://google.com"));
    }
    frontier.prioritize_urls();
}
