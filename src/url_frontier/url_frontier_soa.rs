use rand::{rng, RngExt};

use crate::{
    constants::{
        ALLURLS_CAPACITY, DOMAIN_QUEUE_CAPACITY, HIGH_PRIORITY_DOMAINS, HIGH_PRIORITY_URL_WEIGHT,
        LOW_PRIORITY_URL_WEIGHT, MID_PRIORITY_DOMAINS, MID_PRIORITY_URL_WEIGHT,
        PRIO_QUEUE_CAPACITY, PRIO_QUEUE_INSTANCES,
    },
    url_frontier::priority_queue::PriorityQueue,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct UrlFrontier {
    urls: AllUrls,
    // Priority is in order from highest to lowest priority
    priority_queues: [PriorityQueue; PRIO_QUEUE_INSTANCES],
    domain_queues: Vec<VecDeque<usize>>,
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
            println!(
                "Url '{}' does not appear to be formatted correctly. Skipping priority...",
                full_url
            );
            return 0.3;
        }

        if HIGH_PRIORITY_DOMAINS.contains(&iter[0]) {
            // Place in high prio queue
            println!("url '{}' will be given high priority weight...", full_url);
            return HIGH_PRIORITY_URL_WEIGHT;
        } else if MID_PRIORITY_DOMAINS.contains(&iter[0]) {
            println!("url '{}' will be given mid priority weight...", full_url);
            return MID_PRIORITY_URL_WEIGHT;
        }

        println!("url '{}' will be given mid priority weight...", full_url);
        return LOW_PRIORITY_URL_WEIGHT;
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

    /// Composes a Url struct based on the url index if it hasn't been soft-deleted
    pub fn compose_url_from_idx(&self, url_idx: usize) -> Option<Url> {
        if self.free_slots.contains(&url_idx) {
            return None;
        };
        Some(Url {
            priority_weight: self.priority_weight[url_idx],
            full_url: self.full_url[url_idx].clone(),
            domain_rank: self.domain_rank[url_idx],
            crawl_delay_ms: self.crawl_delay_ms[url_idx],
            is_robots_allowed: self.is_robots_allowed[url_idx],
            requires_javascript: self.requires_javascript[url_idx],
            is_sitemap_url: self.is_sitemap_url[url_idx],
            response_time_ms: self.response_time_ms[url_idx],
        })
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
        if self.free_slots.contains(&idx) {
            return;
        }

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
            ],
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
    /// It does this by following calculating the new theoretical average weight and seeing if that
    /// is greater than the current average weight
    fn allocate_url_to_priority_queue(&mut self, url_idx: usize, priority_weight: f64) {
        assert!(self.priority_queues.len() > 0, "Somehow we have no priority queues!");

        let mut rng = rng();

        let random_idx = rng.random_range(..self.priority_queues.len());

        let lowest_prio_avg_weight: f64 = self.priority_queues[random_idx].avg_priority_weight;
        let new_avg_weight = (self.priority_queues[random_idx].sum_priority_weights
            + priority_weight)
            / (self.priority_queues[random_idx].len() as f64 + 1.0);
        let mut lowest_prio_idx: usize = random_idx;
        let mut largest_gain: f64 = new_avg_weight - lowest_prio_avg_weight;

        for (idx, priority_queue) in &mut self.priority_queues.iter_mut().enumerate() {
            // If a queue is empty always add a url idx
            if priority_queue.len() == 0 {
                lowest_prio_idx = idx;
                break;
            }

            let new_avg_weight = (priority_queue.sum_priority_weights + priority_weight)
                / (priority_queue.len() as f64 + 1.0);
            if new_avg_weight > priority_queue.avg_priority_weight
                && (new_avg_weight - priority_queue.avg_priority_weight) > largest_gain
            {
                lowest_prio_idx = idx;
                largest_gain = new_avg_weight - priority_queue.avg_priority_weight;
            }
        }

        println!("Allocating url with priority_weight '{priority_weight}' to priority_queue {lowest_prio_idx}");
        self.priority_queues[lowest_prio_idx].push_front(url_idx, priority_weight);
    }

    /// Gets the PriorityQueue at queue_idx and replaces it's value with an 
    /// empty PriorityQueue in it's place. Returns the one originally at queue_idx
    pub fn get_priority_queue(&mut self, queue_idx: usize) -> PriorityQueue {
        std::mem::replace(
            &mut self.priority_queues[queue_idx], 
            PriorityQueue::with_capacity(PRIO_QUEUE_CAPACITY),
        )
    }

    fn select_queue_idx(&mut self) -> usize {
        assert!(self.priority_queues.len() > 0, "Somehow we have no priority queues!");

        let mut queue_idx: usize = 0;
        let mut highest_avg_priority: f64 = 0.0;

        for (idx, priority_queue) in self.priority_queues.iter().enumerate() {
            if priority_queue.avg_priority_weight > highest_avg_priority {
                queue_idx = idx;
                highest_avg_priority = priority_queue.avg_priority_weight;
            }
        }

        queue_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{HIGH_PRIORITY_URL_WEIGHT, LOW_PRIORITY_URL_WEIGHT};

    /// Helper: populate both queues with `n` URLs of the given weight directly,
    /// bypassing allocate_url_to_priority_queue so we fully control initial state.
    fn frontier_with_both_queues_populated(weight: f64, n: usize) -> UrlFrontier {
        let mut frontier = UrlFrontier::new();
        for i in 0..n {
            frontier.priority_queues[0].push_front(i, weight);
            frontier.priority_queues[1].push_front(i + n, weight);
        }
        frontier
    }

    /// When both queues are empty, queue[0] is the first empty queue encountered
    /// so the URL should land there alone.
    #[test]
    fn test_empty_queue_gets_filled_first() {
        let mut frontier = UrlFrontier::new();
        frontier.allocate_url_to_priority_queue(0, HIGH_PRIORITY_URL_WEIGHT);

        assert_eq!(frontier.priority_queues[0].len(), 1);
        assert_eq!(frontier.priority_queues[1].len(), 0);
    }

    /// When queue[0] has entries but queue[1] is empty, queue[1] is the first
    /// empty queue encountered so the URL should land there.
    #[test]
    fn test_second_empty_queue_gets_filled_before_gain_comparison() {
        let mut frontier = UrlFrontier::new();
        frontier.priority_queues[0].push_front(0, LOW_PRIORITY_URL_WEIGHT);

        frontier.allocate_url_to_priority_queue(1, HIGH_PRIORITY_URL_WEIGHT);

        assert_eq!(frontier.priority_queues[1].len(), 1);
        assert_eq!(frontier.priority_queues[0].len(), 1); // untouched
    }

    /// A high-weight URL added when both queues have low averages should go to
    /// whichever queue it improves the most. With identical starting averages the
    /// gain is equal so queue[0] wins (it's the default `lowest_prio_idx`).
    #[test]
    fn test_high_weight_url_routed_to_queue_with_largest_gain() {
        let mut frontier = frontier_with_both_queues_populated(LOW_PRIORITY_URL_WEIGHT, 3);

        let before_q0 = frontier.priority_queues[0].len();
        let before_q1 = frontier.priority_queues[1].len();

        frontier.allocate_url_to_priority_queue(99, HIGH_PRIORITY_URL_WEIGHT);

        let added_to_q0 = frontier.priority_queues[0].len() == before_q0 + 1;
        let added_to_q1 = frontier.priority_queues[1].len() == before_q1 + 1;

        assert!(
            added_to_q0 ^ added_to_q1,
            "URL should be added to exactly one priority queue"
        );
    }

    /// A low-weight URL that can't improve any queue's average should fall back
    /// to a randomly chosen queue. Assert exactly one queue grew.
    #[test]
    fn test_low_weight_url_falls_back_to_random_queue() {
        let mut frontier = frontier_with_both_queues_populated(HIGH_PRIORITY_URL_WEIGHT, 3);
        let before_q0 = frontier.priority_queues[0].len();
        let before_q1 = frontier.priority_queues[1].len();

        frontier.allocate_url_to_priority_queue(99, LOW_PRIORITY_URL_WEIGHT);

        let added_to_q0 = frontier.priority_queues[0].len() == before_q0 + 1;
        let added_to_q1 = frontier.priority_queues[1].len() == before_q1 + 1;

        assert!(
            added_to_q0 ^ added_to_q1,
            "Low-weight URL should fall back to exactly one priority queue"
        );
    }

    /// After allocation, sum_priority_weights and avg_priority_weight on the
    /// receiving queue must reflect the newly added URL.
    #[test]
    fn test_queue_metadata_updated_after_allocation() {
        let mut frontier = UrlFrontier::new();
        // Seed both queues so no early-return fires.
        frontier.priority_queues[0].push_front(0, LOW_PRIORITY_URL_WEIGHT);
        frontier.priority_queues[1].push_front(1, LOW_PRIORITY_URL_WEIGHT);

        frontier.allocate_url_to_priority_queue(2, HIGH_PRIORITY_URL_WEIGHT);

        // Find whichever queue received the new URL.
        let q = if frontier.priority_queues[0].len() == 2 {
            &frontier.priority_queues[0]
        } else {
            &frontier.priority_queues[1]
        };

        let expected_sum = LOW_PRIORITY_URL_WEIGHT + HIGH_PRIORITY_URL_WEIGHT;
        let expected_avg = expected_sum / 2.0;

        assert!(
            (q.sum_priority_weights - expected_sum).abs() < f64::EPSILON,
            "sum_priority_weights should be {expected_sum}, got {}",
            q.sum_priority_weights
        );
        assert!(
            (q.avg_priority_weight - expected_avg).abs() < f64::EPSILON,
            "avg_priority_weight should be {expected_avg}, got {}",
            q.avg_priority_weight
        );
    }
}
