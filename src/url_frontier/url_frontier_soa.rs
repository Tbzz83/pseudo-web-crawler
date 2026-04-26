use rand::{rng, RngExt};
use std_semaphore::Semaphore;
use tokio::sync::{mpsc::{channel, Receiver, Sender}, RwLock};

use crate::{
    constants::{
        ALLURLS_CAPACITY, DOMAIN_QUEUE_CAPACITY, HIGH_PRIORITY_DOMAINS, HIGH_PRIORITY_URL_WEIGHT,
        LOW_PRIORITY_URL_WEIGHT, MID_PRIORITY_DOMAINS, MID_PRIORITY_URL_WEIGHT,
        PRIO_QUEUE_CAPACITY, PRIO_QUEUE_INSTANCES,
    },
    url_frontier::priority_queue::{self, Priority, PriorityQueue},
};
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;


#[derive(Debug)]
pub struct Url {
    priority: Priority,

    full_url: String,
    //    path: String,
    //    query_params: String,
    //    fragment: String,

    // Metadata fields with varying sizes (creates padding opportunities)
    //    priority: u8,
    //    is_visited: bool,
    //    depth: u16,
    //    retry_count: u8,
    //    is_external: bool,
    //    status_code: u16,
    //    content_length: u64,
    //    last_crawled_timestamp: u64,
    //    checksum: u32,
    domain_rank: u32,
    crawl_delay_ms: u16,
    is_robots_allowed: bool,
    requires_javascript: bool,
    is_sitemap_url: bool,
    response_time_ms: u32,
}

impl Url {
    pub fn new(full_url: &str) -> Url {
        Url {
            priority: Self::prioritize_url(full_url),
            full_url: full_url.to_string(),
            domain_rank: 0,
            crawl_delay_ms: 0,
            is_robots_allowed: true,
            requires_javascript: false,
            is_sitemap_url: false,
            response_time_ms: 0,
        }
    }
    /// Calculates a priority based on the url domain name
    fn prioritize_url(full_url: &str) -> Priority {
        let iter = full_url.split(".").collect::<Vec<&str>>();

        // the zeroth element was consumed by above line
        // so we call nth(0) again to actually get the string
        // at index 1
        if iter.len() == 1 {
            println!(
                "Url '{}' does not appear to be formatted correctly. Skipping priority...",
                full_url
            );
            return Priority::Low;
        }

        if HIGH_PRIORITY_DOMAINS.contains(&iter[0]) {
            // Place in high prio queue
            println!("url '{}' will be given high priority weight...", full_url);
            return Priority::High;
        } else if MID_PRIORITY_DOMAINS.contains(&iter[0]) {
            println!("url '{}' will be given mid priority weight...", full_url);
            return Priority::Medium;
        }

        println!("url '{}' will be given mid priority weight...", full_url);
        Priority::Low
    }
}

#[derive(Debug)]
struct AllUrls {
    size: usize,
    priority: Vec<Priority>,
    /// Vector indicating indexes that should be overwritten. ie. soft-deleted
    free_slots: Vec<usize>,
    full_url: Vec<String>,
    //    path: Vec<String>,
    //    query_params: Vec<String>,
    //    fragment: Vec<String>,
    //
    //    // Metadata fields with varying sizes (creates padding opportunities)
    //    priority: Vec<u8>,
    //    is_visited: bool,
    //    depth: u16,
    //    retry_count: Vec<u8>,
    //    is_external: bool,
    //    status_code: u16,
    //    content_length: u64,
    //    last_crawled_timestamp: u64,
    //    checksum: u32,
    domain_rank: Vec<u32>,
    crawl_delay_ms: Vec<u16>,
    is_robots_allowed: Vec<bool>,
    requires_javascript: Vec<bool>,
    is_sitemap_url: Vec<bool>,
    response_time_ms: Vec<u32>,
}

impl AllUrls {
    pub fn new() -> AllUrls {
        Self::with_capacity(ALLURLS_CAPACITY)
    }

    pub fn get_full_url(&self, url_idx: usize) -> Option<String> {
        if let Some(url) = self.full_url.get(url_idx) {
            return Some(url.to_owned());
        }

        None
    }

    /// Composes a Url struct based on the url index if it hasn't been soft-deleted
    /// Soft-delete the url index
    pub fn compose_url_from_idx(&mut self, url_idx: usize) -> Option<Url> {
        if self.free_slots.contains(&url_idx) {
            return None;
        };
        let url = Some(Url {
            priority: self.priority[url_idx].clone(),
            full_url: self.full_url[url_idx].clone(),
            domain_rank: self.domain_rank[url_idx],
            crawl_delay_ms: self.crawl_delay_ms[url_idx],
            is_robots_allowed: self.is_robots_allowed[url_idx],
            requires_javascript: self.requires_javascript[url_idx],
            is_sitemap_url: self.is_sitemap_url[url_idx],
            response_time_ms: self.response_time_ms[url_idx],
        });

        self.remove(url_idx);

        url
    }

    pub fn with_capacity(capacity: usize) -> AllUrls {
        AllUrls {
            size: 0,
            priority: Vec::with_capacity(capacity),
            free_slots: vec![],
            full_url: Vec::with_capacity(capacity),
            //            path: String::new(),
            //            query_params: String::new(),
            //            fragment: String::new(),
            //            priority: 0,
            //            is_visited: false,
            //            depth: 0,
            //            retry_count: 0,
            //            is_external: false,
            //            status_code: 0,
            //            content_length: 0,
            //            last_crawled_timestamp: 0,
            //            checksum: 0,
            domain_rank: Vec::with_capacity(capacity),
            crawl_delay_ms: Vec::with_capacity(capacity),
            is_robots_allowed: Vec::with_capacity(capacity),
            requires_javascript: Vec::with_capacity(capacity),
            is_sitemap_url: Vec::with_capacity(capacity),
            response_time_ms: Vec::with_capacity(capacity),
        }
    }

    /// Adds a new url, either using a free slot or by pushing onto the end
    /// of AllUrls
    pub fn add(&mut self, url: &Url) -> usize {
        let url_idx: usize;
        if let Some(free_idx) = self.free_slots.pop() {
            self.priority[free_idx] = url.priority.clone();
            self.full_url[free_idx] = url.full_url.clone();
            self.domain_rank[free_idx] = url.domain_rank;
            self.crawl_delay_ms[free_idx] = url.crawl_delay_ms;
            self.is_robots_allowed[free_idx] = url.is_robots_allowed;
            self.requires_javascript[free_idx] = url.requires_javascript;
            self.is_sitemap_url[free_idx] = url.is_sitemap_url;
            self.response_time_ms[free_idx] = url.response_time_ms;
            url_idx = free_idx;
        } else {
            url_idx = self.size;
            self.priority.push(url.priority.clone());
            self.full_url.push(url.full_url.clone());
            self.domain_rank.push(url.domain_rank);
            self.crawl_delay_ms.push(url.crawl_delay_ms);
            self.is_robots_allowed.push(url.is_robots_allowed);
            self.requires_javascript.push(url.requires_javascript);
            self.is_sitemap_url.push(url.is_sitemap_url);
            self.response_time_ms.push(url.response_time_ms);
            self.size += 1;
        }

        url_idx
    }

    /// Soft-deletes a Url from the UrlFrontier. This function will
    /// allow this idx to be used by a new Url when another is pushed
    pub fn remove(&mut self, idx: usize) {
        if self.free_slots.contains(&idx) {
            panic!("Double free of url index");
        }

        self.free_slots.push(idx);
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive()]
pub struct UrlFrontier {
    urls: Arc<Mutex<AllUrls>>,
    // Priority is in order from highest to lowest priority

    priority_queues: Option<Arc<Mutex<[PriorityQueue; PRIO_QUEUE_INSTANCES]>>>,
//    priority_queues_senders: Option<[Sender<usize>; PRIO_QUEUE_INSTANCES]>,
//    priority_queues_receivers: Option<[Receiver<usize>; PRIO_QUEUE_INSTANCES]>,
    priority_queues_notify_sem: Arc<Semaphore>,

    domain_queues: Vec<VecDeque<usize>>,
}

impl UrlFrontier {
    pub async fn new() -> UrlFrontier {
        Self::with_capacity(ALLURLS_CAPACITY).await
    }


    pub async fn with_capacity(capacity: usize) -> UrlFrontier {
        let urls: AllUrls = AllUrls::with_capacity(capacity);
        
        let mut q1 = PriorityQueue::with_capacity(PRIO_QUEUE_CAPACITY, Priority::High);
        let mut q2 = PriorityQueue::with_capacity(PRIO_QUEUE_CAPACITY, Priority::Medium);
        let mut q3 = PriorityQueue::with_capacity(PRIO_QUEUE_CAPACITY, Priority::Low);

        let notify_sem = Arc::new(Semaphore::new(0));


        UrlFrontier {
            urls: Arc::new(Mutex::new(urls)),

            priority_queues: Some(Arc::new(Mutex::new([
                q1,
                q2,
                q3,
            ]))),

            domain_queues: vec![
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
            ],

            priority_queues_notify_sem: notify_sem,
        }
    }

    async fn priority_queues_process_urls(&mut self) {
        let notify_sem = self.priority_queues_notify_sem.clone();
        let all_urls = self.urls.clone();
        if let Some(priority_queues) = self.priority_queues.clone() {
            tokio::spawn(async move {
                loop {
                    notify_sem.acquire();
                    let mut guard = priority_queues.lock().await;

                    for (idx, priority_queue) in guard.iter_mut().enumerate() {
                        if let Some(is_empty) = priority_queue.is_empty() {
                            if is_empty {
                                //println!("Priority queue {idx} is empty");
                                continue;
                            }

                            // Now we have a receiver with something in it
                            if let Some(url_idx) = priority_queue.pop_left().await {
                                println!("Received a url index: {:?}", url_idx);
                                
                                // TODO
                                // process URLS send to back queue router
                            }
                        } else {
                            println!("No receiver found for priority queue {idx}");
                        }
                    }
                }
            });
        } else {
            panic!("Priority queues is None somehow!");
        }



    }

    pub async fn run(
        &mut self,
    )
    {
        self.priority_queues_process_urls().await;
    }

    /// Pushes a url onto the frontier, and returns its index in the frontier.
    pub async fn add_url(&mut self, url: Url) -> usize {
        let url_idx = self.urls.lock().await.add(&url);

        match url.priority {
            Priority::High => {
                if let Some(priority_queues) = self.priority_queues.as_mut() {
                    priority_queues.lock().await[0].push(url_idx).await;
                } else {
                    panic!("Error: Priority queues not set");
                }
            },
            Priority::Medium => {
                if let Some(priority_queues) = self.priority_queues.as_mut() {
                    priority_queues.lock().await[1].push(url_idx).await;
                } else {
                    panic!("Priority queues not set");
                }
            },
            Priority::Low => {
                if let Some(priority_queues) = self.priority_queues.as_mut() {
                    priority_queues.lock().await[2].push(url_idx).await;
                } else {
                    panic!("Priority queues not set");
                }
            },
        } 

        let notify_sem = self.priority_queues_notify_sem.clone();
        notify_sem.release();

        url_idx
    }
}
