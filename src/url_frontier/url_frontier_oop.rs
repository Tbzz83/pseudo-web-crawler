use std::fmt::Debug;

#[derive(Debug)]
pub struct UrlFrontier {
    urls_to_prioritize: Vec<Option<Box<dyn FunctionalUrl>>>,
}

trait FunctionalUrl
where
    Self: Debug,
{
    fn sum_domain_rank(&mut self);
    fn get_domain_rank(&self) -> u32;
}

#[derive(Debug)]
pub struct Url {
    host_name: String,
    //    path: String,
    //    query_params: String,
    //    fragment: String,
    //
    //    // Metadata fields with varying sizes (creates padding opportunities)
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
            domain_rank: 0,
            crawl_delay_ms: 0,
            is_robots_allowed: true,
            requires_javascript: false,
            is_sitemap_url: false,
            response_time_ms: 0,
        }
    }
}

impl FunctionalUrl for Url {
    fn sum_domain_rank(&mut self) {
        let mut sum: u64 = 0;
        sum = sum.wrapping_add(self.domain_rank as u64);
        self.domain_rank = sum as u32;
    }

    fn get_domain_rank(&self) -> u32 {
        self.domain_rank
    }
}

const DEFAULT_CAPACITY: usize = 100000;

impl UrlFrontier {
    pub fn new() -> UrlFrontier {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> UrlFrontier {
        let urls_to_prioritize: Vec<Option<Box<dyn FunctionalUrl>>> = Vec::with_capacity(capacity);
        UrlFrontier { urls_to_prioritize }
    }

    pub fn push_url(&mut self, url: Url) {
        self.urls_to_prioritize.push(Some(Box::new(url)));
    }

    pub fn prioritize_urls(&mut self) -> u64 {
        let mut sum: u64 = 0;
        for opt_url in &mut self.urls_to_prioritize {
            let Some(url) = opt_url else {
                continue;
            };
            url.as_mut().sum_domain_rank();
            sum = sum.wrapping_add(url.get_domain_rank() as u64);
        }
        sum
    }
}
