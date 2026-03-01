use std::collections::VecDeque;

/// NOTE
/// The plan for this is that this struct will have a vec deque.
/// BUT, crucially it also contains some metadata about the queue,
/// such as how many high prioritization urls it contains. Or we're
/// going to have to have some value/weight that indicates its priority.
/// The prioritizer is going to basically loop through all our PriorityQueue
/// structs, and push_front high priority urls to queues with the LOWEST (configurable)
/// priority, to keep things fair

#[derive(Debug)]
pub struct PriorityQueue {
    queue: VecDeque<usize>,

    /// value between 0 - 1. The closer it is to
    /// 1, the higher its priority
    pub avg_priority_weight: f64,
    pub sum_priority_weights: f64,
}

impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        PriorityQueue {
            queue: VecDeque::new(),
            avg_priority_weight: 0.0,
            sum_priority_weights: 0.0,
        }
    }

    pub fn with_capacity(capacity: usize) -> PriorityQueue {
        PriorityQueue {
            queue: VecDeque::with_capacity(capacity),
            avg_priority_weight: 0.0,
            sum_priority_weights: 0.0,
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn push_front(&mut self, url_idx: usize, priority_weight: f64) {
        self.queue.push_front(url_idx);
        self.sum_priority_weights += priority_weight;
        self.calculate_avg_priority_weights();
    }

    fn calculate_avg_priority_weights(&mut self) {
        if self.queue.len() > 0 {
            self.avg_priority_weight = self.sum_priority_weights / self.queue.len() as f64;
        }
    }
}
