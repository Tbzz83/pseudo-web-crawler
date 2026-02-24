const DEFAULT_CAPACITY: usize = 100000;

#[derive(Debug)]
pub struct UrlFrontier {
    _urls_to_prioritize: AllUrls,
}

#[derive(Debug)]
pub struct Url {
    _host_name: String,
    //    _path: String,
    //    _query_params: String,
    //    _fragment: String,

    // Metadata fields with varying sizes (creates padding opportunities)
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
    _crawl_delay_ms: u16,
    _is_robots_allowed: bool,
    _requires_javascript: bool,
    _is_sitemap_url: bool,
    _response_time_ms: u32,
}

impl Url {
    pub fn new(host_name: &str) -> Url {
        Url {
            _host_name: host_name.to_string(),
            _domain_rank: 0,
            _crawl_delay_ms: 0,
            _is_robots_allowed: true,
            _requires_javascript: false,
            _is_sitemap_url: false,
            _response_time_ms: 0,
        }
    }
}

#[derive(Debug)]
pub struct AllUrls {
    _size: usize,
    _host_name: Vec<String>,
    //    _path: Vec<String>,
    //    _query_params: Vec<String>,
    //    _fragment: Vec<String>,
    //
    //    // Metadata fields with varying sizes (creates padding opportunities)
    //    _priority: Vec<u8>,
    //    _is_visited: bool,
    //    _depth: u16,
    //    _retry_count: Vec<u8>,
    //    _is_external: bool,
    //    _status_code: u16,
    //    _content_length: u64,
    //    _last_crawled_timestamp: u64,
    //    _checksum: u32,
    _domain_rank: Vec<u32>,
    _crawl_delay_ms: Vec<u16>,
    _is_robots_allowed: Vec<bool>,
    _requires_javascript: Vec<bool>,
    _is_sitemap_url: Vec<bool>,
    _response_time_ms: Vec<u32>,
}

impl AllUrls {
    pub fn new() -> AllUrls {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> AllUrls {
        AllUrls {
            _size: 0,
            _host_name: Vec::with_capacity(capacity),
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
            _domain_rank: Vec::with_capacity(capacity),
            _crawl_delay_ms: Vec::with_capacity(capacity),
            _is_robots_allowed: Vec::with_capacity(capacity),
            _requires_javascript: Vec::with_capacity(capacity),
            _is_sitemap_url: Vec::with_capacity(capacity),
            _response_time_ms: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, url: Url) {
        self._size += 1;
        self._host_name.push(url._host_name);
        self._domain_rank.push(url._domain_rank);
        self._crawl_delay_ms.push(url._crawl_delay_ms);
        self._is_robots_allowed.push(url._is_robots_allowed);
        self._requires_javascript.push(url._requires_javascript);
        self._is_sitemap_url.push(url._is_sitemap_url);
        self._response_time_ms.push(url._response_time_ms);
    }

    pub fn size(&self) -> usize {
        self._size
    }
}

impl UrlFrontier {
    pub fn new() -> UrlFrontier {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> UrlFrontier {
        let urls_to_prioritize: AllUrls = AllUrls::with_capacity(capacity);
        UrlFrontier {
            _urls_to_prioritize: urls_to_prioritize,
        }
    }

    pub fn push_url(&mut self, url: Url) -> usize {
        self._urls_to_prioritize.push(url);
        self._urls_to_prioritize.size()
    }

    pub fn prioritize_urls(&self) -> u64 {
        let mut sum: u64 = 0;
        for dom_rank in &self._urls_to_prioritize._domain_rank {
            sum = sum.wrapping_add(*dom_rank as u64);
        }
        sum
    }
}
