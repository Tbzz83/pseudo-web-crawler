use std::collections::VecDeque;
use crate::{constants::{ALLURLS_CAPACITY, DOMAIN_QUEUE_CAPACITY, HIGH_PRIORITY_DOMAINS, HIGH_PRIORITY_URL_WEIGHT, LOW_PRIORITY_URL_WEIGHT, PRIO_QUEUE_CAPACITY, PRIO_QUEUE_INSTANCES}, url_frontier::priority_queue::PriorityQueue};

#[derive(Debug)]
pub struct UrlFrontier {
    urls: AllUrls,
    // Priority is in order from highest to lowest priority
    priority_queues: [PriorityQueue; PRIO_QUEUE_INSTANCES],
    domain_queues: Vec<VecDeque<usize>>
}

#[derive(Debug)]
pub struct Url {
    /// value between 0 - 1. The closer it is to 
    /// 1, the higher its priority
    priority_weight: f64,

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
            priority_weight: Self::prioritize_url(full_url),
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
    fn prioritize_url(full_url: &str) -> f64 {
        let iter = full_url.split(".").collect::<Vec<&str>>();

        // the zeroth element was consumed by above line
        // so we call nth(0) again to actually get the string
        // at index 1
        if iter.len() == 1 {
            println!("Url '{}' does not appear to be formatted correctly. Skipping priority...", full_url);
            return 0.3
        }

        if HIGH_PRIORITY_DOMAINS.contains(&iter[0]) {
            // Place in high prio queue
            println!("url '{}' will be sent to high priority queue...", full_url);
            return HIGH_PRIORITY_URL_WEIGHT 
        } else {
            println!("url '{}' will be sent to low priority queue...", full_url);
            return LOW_PRIORITY_URL_WEIGHT
        }
    }
}


#[derive(Debug)]
pub struct AllUrls {
    size: usize,
    priority_weight: Vec<f64>,
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

    pub fn with_capacity(capacity: usize) -> AllUrls {
        AllUrls {
            size: 0,
            priority_weight: Vec::with_capacity(capacity),
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
            self.full_url[free_idx] = url.full_url.clone();
            self.domain_rank[free_idx] = url.domain_rank;
            self.crawl_delay_ms[free_idx] = url.crawl_delay_ms;
            self.is_robots_allowed[free_idx] = url.is_robots_allowed;
            self.requires_javascript[free_idx] = url.requires_javascript;
            self.is_sitemap_url[free_idx] = url.is_sitemap_url;
            self.response_time_ms[free_idx] = url.response_time_ms;
            url_idx = free_idx;
        } else {
            self.size += 1;
            self.full_url.push(url.full_url.clone());
            self.domain_rank.push(url.domain_rank);
            self.crawl_delay_ms.push(url.crawl_delay_ms);
            self.is_robots_allowed.push(url.is_robots_allowed);
            self.requires_javascript.push(url.requires_javascript);
            self.is_sitemap_url.push(url.is_sitemap_url);
            self.response_time_ms.push(url.response_time_ms);
            url_idx = self.size
        }

        url_idx
    }

    /// Soft-deletes a Url from the UrlFrontier. This function will
    /// allow this idx to be used by a new Url when another is pushed
    pub fn remove(&mut self, idx: usize) {
        self.free_slots.push(idx);
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl UrlFrontier {
    pub fn new() -> UrlFrontier {
        Self::with_capacity(ALLURLS_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> UrlFrontier {
        let urls: AllUrls = AllUrls::with_capacity(capacity);
        UrlFrontier { 
            urls: urls,
            priority_queues: [
                PriorityQueue::with_capacity(PRIO_QUEUE_CAPACITY),
                PriorityQueue::with_capacity(PRIO_QUEUE_CAPACITY),
            ],
            domain_queues: vec![
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
            ]
        }
    }

    /// Pushes a url onto the frontier, and returns its index in the frontier.
    pub fn add_url(&mut self, url: Url) -> usize {
        let url_idx = self.urls.add(&url);
        self.allocate_url_to_priority_queue(url_idx, url.priority_weight);
        url_idx
    }

    /// The current allocation method loops through all our priority queues, and 
    /// tries to add the new url such that it increases the priority_queue.avg_prioritiy_weights.
    /// It does this by following the formula `(new_priority_weight +
    /// sum_priority_weights/(priority_queue.len() + 1) > avg_priority_weight`
    /// => new_priority_weight > avg_priority_weight - (sum_priority_weights/priority_queue.len()) - 1
    fn allocate_url_to_priority_queue(&mut self, url_idx: usize, priority_weight: f64) {
        if self.priority_queues.len() == 0 {
            panic!("Somehow priority queues are empty!");
        }

        for priority_queue in &mut self.priority_queues {
            if priority_weight > priority_queue.avg_priority_weight - (priority_queue.sum_priority_weights/priority_queue.len() as f64) - 1.0 {
                // Adding this priority_weight improves the avg so we will do it.
                priority_queue.push_front(url_idx, priority_weight);
                return
            }
        }

        // If the priority_weight doesn't improve any priority queues average_priority_weight, just
        // assign it to the first priority_queue. In future, randomize. TODO
        self.priority_queues[0].push_front(url_idx, priority_weight);
    }
}













