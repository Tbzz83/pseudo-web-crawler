use crate::url_frontier::url_frontier_soa::{Url, UrlFrontier};

mod constants;
mod url_frontier;

fn main() {
    let mut frontier = UrlFrontier::new();

    // High priority
    frontier.add_url(Url::new("google.com"));
    frontier.add_url(Url::new("youtube.com"));
    frontier.add_url(Url::new("wikipedia.com"));
    frontier.add_url(Url::new("github.com"));
    frontier.add_url(Url::new("reddit.com"));
    frontier.add_url(Url::new("stackoverflow.com"));

    // Mid priority
    frontier.add_url(Url::new("medium.com"));
    frontier.add_url(Url::new("hackernews.com"));
    frontier.add_url(Url::new("nytimes.com"));

    // Low priority
    frontier.add_url(Url::new("yahoo.com"));
    frontier.add_url(Url::new("blogspot.com"));
    frontier.add_url(Url::new("tumblr.com"));
    frontier.add_url(Url::new("somerandomblog.com"));

    dbg!(frontier);
}
