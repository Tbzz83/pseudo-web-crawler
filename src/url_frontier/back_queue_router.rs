use std::{collections::{HashMap, VecDeque}, sync::Arc};

use tokio::sync::Mutex;

use crate::{constants::DOMAIN_QUEUE_CAPACITY, url_frontier::url_frontier_soa::Url};

pub struct BackQueueRouter;

impl BackQueueRouter {
    pub async fn process_url(url_idx: usize, domain_queues_guard: Arc<Mutex<HashMap<String, VecDeque<usize>>>>, url_domain_name: &str) {
        println!("Receieved a url in the back queue router: {:?}", url_idx);

        let mut domain_queues = domain_queues_guard.lock().await;

        if !domain_queues.contains_key(url_domain_name) {
            domain_queues.insert(
                String::from(url_domain_name),
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY)
            );
        }
    }
}
