use std::{collections::VecDeque, sync::{Arc, atomic::AtomicI32}};

use tokio::sync::{RwLock, mpsc::{self, Receiver, Sender, channel}};

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
    tx: Sender<usize>,
    rx: Receiver<usize>,
    /// value between 0 - 1. The closer it is to
    /// 1, the higher its priority
    avg_priority_weight: Arc<RwLock<f64>>,
    count: Arc<RwLock<usize>>,
}

impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        Self::with_capacity(32)
    }

    pub fn with_capacity(capacity: usize) -> PriorityQueue {
        let (tx, rx) = channel::<usize>(capacity);
        PriorityQueue {
            tx,
            rx,
            count: Arc::new(RwLock::new(0)),
            avg_priority_weight: Arc::new(RwLock::new(0.00)),
        }
    }

    pub async fn push(&mut self, url_idx: usize, priority_weight: f64) {
        let tx_clone = self.tx.clone();
        let avg_lock = self.avg_priority_weight.clone();
        let count_lock = self.count.clone();

        tokio::spawn(async move {
            let mut avg_priority_weight = avg_lock.write().await;
            let mut count = count_lock.write().await;
            *avg_priority_weight = ((*avg_priority_weight * *count as f64) + priority_weight)/(*count as f64 + 1.0);
            *count += 1;
            tx_clone.send(url_idx);
        });
    }

    pub async fn pop(&mut self) -> Option<usize> {
        todo!("Need a lookup priority_weight given url_idx");
        if let Some(url_idx) = self.rx.recv().await {
            let mut avg_priority_weight = self.avg_priority_weight.write().await;
            let mut count = self.count.write().await;
            *avg_priority_weight = ((*avg_priority_weight * *count as f64) - priority_weight)/(*count as f64 - 1.0);
            *count -= 1;
            return Some(url_idx)
        }

        None
    }
}
