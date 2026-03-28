/// Starting capacity of AllURls SoA object
pub const ALLURLS_CAPACITY: usize = 100000;

/// How many separate prioritization queues can exist
pub const PRIO_QUEUE_INSTANCES: usize = 3;

/// How many separate domain queues can exist
//pub const DOMAIN_QUEUE_INSTANCES: usize = 3;

/// Starting capacity of prioritization VecDeque objects
pub const PRIO_QUEUE_CAPACITY: usize = 25000;
/// Starting capacity of domain VecDeque objects
pub const DOMAIN_QUEUE_CAPACITY: usize = 25000;

/// Max number of elements in the queue before it gets sent to back queue router
pub const PRIO_QUEUE_LIMIT: usize = 10;
/// Max number of elements in the queue before it gets sent to back queue router
pub const DOMAIN_QUEUE_LIMIT: usize = 10;

pub const HIGH_PRIORITY_QUEUE_IDX: usize = 0;
pub const MID_PRIORITY_QUEUE_IDX: usize = 1;
pub const LOW_PRIORITY_QUEUE_IDX: usize = 1;

pub const HIGH_PRIORITY_DOMAINS: &[&str] = &[
    "google",
    "youtube",
    "wikipedia",
    "github",
    "reddit",
    "stackoverflow",
];
pub const MID_PRIORITY_DOMAINS: &[&str] = &["medium", "hackernews", "nytimes", "bbc"];
pub const LOW_PRIORITY_DOMAINS: &[&str] = &["blogspot", "weebly", "wixsite", "tumblr"];

pub const HIGH_PRIORITY_URL_WEIGHT: f64 = 0.8;
pub const MID_PRIORITY_URL_WEIGHT: f64 = 0.5;
pub const LOW_PRIORITY_URL_WEIGHT: f64 = 0.2;
