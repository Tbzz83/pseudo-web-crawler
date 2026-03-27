use crate::url_frontier::url_frontier_soa::{Url, UrlFrontier};
use tokio::{join, sync::mpsc::{self, Receiver, Sender}, task::JoinHandle, time::sleep};

mod constants;
mod url_frontier;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
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

    dbg!(frontier.get_highest_priority_queue());
    dbg!(frontier);
}
