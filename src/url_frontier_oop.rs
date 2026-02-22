use std::fmt::Debug;

#[derive(Debug)]
pub struct UrlFrontier {
    _urls_to_prioritize: Vec<Option<Box<dyn FunctionalUrl>>>,
}

trait FunctionalUrl 
where Self: Debug {
    fn sum_domain_rank(&mut self);
    fn get_domain_rank(&self) -> u32;
}

#[derive(Debug)]
pub struct Url {
//    _host_name: String,
//    _path: String,
//    _query_params: String,
//    _fragment: String,
//
//    // Metadata fields with varying sizes (creates padding opportunities)
//    _priority: u8,
//    _is_visited: bool,
//    _depth: u16,
//    _retry_count: u8,
//    _is_external: bool,
//    _status_code: u16,
//    _content_length: u64,
//    _last_crawled_timestamp: u64,
//    _checksum: u32,
    _domain_rank: u32,
//    _crawl_delay_ms: u16,
//    _is_robots_allowed: bool,
//    _requires_javascript: bool,
//    _is_sitemap_url: bool,
//    _response_time_ms: u32,
}

impl Url {
    pub fn new(host_name: &str) -> Url {
        Url {
//            _host_name: host_name.to_string(),
//            _path: String::new(),
//            _query_params: String::new(),
//            _fragment: String::new(),
//            _priority: 0,
//            _is_visited: false,
//            _depth: 0,
//            _retry_count: 0,
//            _is_external: false,
//            _status_code: 0,
//            _content_length: 0,
//            _last_crawled_timestamp: 0,
//            _checksum: 0,
            _domain_rank: 0,
//            _crawl_delay_ms: 0,
//            _is_robots_allowed: true,
//            _requires_javascript: false,
//            _is_sitemap_url: false,
//            _response_time_ms: 0,
        }
    }
}

impl FunctionalUrl for Url {
    fn sum_domain_rank(&mut self) {
        let mut sum: u64 = 0;
        sum = sum.wrapping_add(self._domain_rank as u64);
        self._domain_rank = sum as u32;
    }

    fn get_domain_rank(&self) -> u32 {
        self._domain_rank
    }
}

impl UrlFrontier {
    pub fn new() -> UrlFrontier {
        let urls_to_prioritize: Vec<Option<Box<dyn FunctionalUrl>>> = Vec::with_capacity(1000000);
        UrlFrontier {
            _urls_to_prioritize: urls_to_prioritize,
        }
    }

    pub fn push_url(&mut self, url: Url) {
        self._urls_to_prioritize.push(Some(Box::new(url)));
    }

    pub fn prioritize_urls(&mut self) -> u64 {
        let mut sum: u64 = 0;
        for opt_url in &mut self._urls_to_prioritize {
            let Some(url) = opt_url else {
                continue;
            };
            url.as_mut().sum_domain_rank();
            sum = sum.wrapping_add(url.get_domain_rank() as u64);
        }
        sum
    }
}
