use crate::url_frontier::url_frontier_soa::{Url, UrlFrontier};
use tokio::{join, sync::mpsc::{self, Receiver, Sender}, task::JoinHandle, time::sleep};

mod constants;
mod url_frontier;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let mut frontier = UrlFrontier::new().await;
    frontier.run().await;

    // Mid priority
    frontier.add_url(Url::new("medium.com")).await;
    frontier.add_url(Url::new("hackernews.com")).await;
    frontier.add_url(Url::new("nytimes.com")).await;

    // Low priority
//    frontier.add_url(Url::new("yahoo.com")).await;
//    frontier.add_url(Url::new("blogspot.com")).await;
//    frontier.add_url(Url::new("tumblr.com")).await;
//    frontier.add_url(Url::new("somerandomblog.com")).await;

    // High priority
    frontier.add_url(Url::new("google.com")).await;
//    frontier.add_url(Url::new("youtube.com")).await;
//    frontier.add_url(Url::new("wikipedia.com")).await;
//    frontier.add_url(Url::new("github.com")).await;
//    frontier.add_url(Url::new("reddit.com")).await;
//    frontier.add_url(Url::new("stackoverflow.com")).await;

    //dbg!(frontier);
    loop {

    }
}
