use crate::url_frontier_oop::{Url as UrlOop, UrlFrontier as UrlFrontierOop};
use crate::url_frontier_aos::{Url as UrlAos, UrlFrontier as UrlFrontierAos};
pub mod url_frontier_oop;
pub mod url_frontier_aos;

struct ComplexObj {
    name: String,
    color: String,
    position_x: i32,
    position_y: i32,
}

fn main() {
    let mut frontier = UrlFrontierAos::new();
    for _ in 0..1000000 {
        frontier.push_url(UrlAos::new("https://google.com"));
    }
    frontier.prioritize_urls();
}
