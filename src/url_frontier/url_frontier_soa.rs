use std::collections::VecDeque;

/// Starting capacity of AllURls SoA object
const ALLURLS_CAPACITY: usize = 100000;
/// How many separate prioritization queues can exist
const PRIO_QUEUE_INSTANCES: usize = 3;
/// How many separate domain queues can exist
//const DOMAIN_QUEUE_INSTANCES: usize = 3;
/// Starting capacity of prioritization VecDeque objects
const PRIO_QUEUE_CAPACITY: usize = 25000;
/// Starting capacity of domain VecDeque objects
const DOMAIN_QUEUE_CAPACITY: usize = 25000;

#[derive(Debug)]
pub struct UrlFrontier {
    urls_to_prioritize: AllUrls,
    // Priority is in order from highest to lowest priority
    prioritization_queues: [VecDeque<usize>; PRIO_QUEUE_INSTANCES],
    domain_queues: Vec<VecDeque<usize>>
}

#[derive(Debug)]
pub struct Url {
    host_name: String,
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
    pub fn new(host_name: &str) -> Url {
        Url {
            host_name: host_name.to_string(),
            domain_rank: 0,
            crawl_delay_ms: 0,
            is_robots_allowed: true,
            requires_javascript: false,
            is_sitemap_url: false,
            response_time_ms: 0,
        }
    }
}

#[derive(Debug)]
pub struct AllUrls {
    size: usize,
    /// Vector indicating indexes that should be overwritten. ie. soft-deleted
    free_slots: Vec<usize>,
    host_name: Vec<String>,
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
            free_slots: vec![],
            host_name: Vec::with_capacity(capacity),
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
            self.host_name[free_idx] = url.host_name.clone();
            self.domain_rank[free_idx] = url.domain_rank;
            self.crawl_delay_ms[free_idx] = url.crawl_delay_ms;
            self.is_robots_allowed[free_idx] = url.is_robots_allowed;
            self.requires_javascript[free_idx] = url.requires_javascript;
            self.is_sitemap_url[free_idx] = url.is_sitemap_url;
            self.response_time_ms[free_idx] = url.response_time_ms;
            url_idx = free_idx;
        } else {
            self.size += 1;
            self.host_name.push(url.host_name.clone());
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
        let urls_to_prioritize: AllUrls = AllUrls::with_capacity(capacity);
        UrlFrontier { 
            urls_to_prioritize: urls_to_prioritize,
            prioritization_queues: [
                VecDeque::with_capacity(PRIO_QUEUE_CAPACITY),
                VecDeque::with_capacity(PRIO_QUEUE_CAPACITY),
                VecDeque::with_capacity(PRIO_QUEUE_CAPACITY),
            ],
            domain_queues: vec![
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
                VecDeque::with_capacity(DOMAIN_QUEUE_CAPACITY),
            ]
        }
    }

    // Pushes a url onto the frontier, and returns its index in the frontier.
    pub fn add_url(&mut self, url: Url) -> usize {
        let url_idx = self.urls_to_prioritize.add(&url);
        self.prioritize_url(&url);
        url_idx
    }

    fn prioritize_url(&self, url: &Url) {

    }


    pub fn prioritize_urls(&self) -> u64 {
        let mut sum: u64 = 0;
        for dom_rank in &self.urls_to_prioritize.domain_rank {
            sum = sum.wrapping_add(*dom_rank as u64);
        }
        sum
    }
}
