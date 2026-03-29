use std::{collections::VecDeque, sync::{Arc, atomic::AtomicI32}};

use std_semaphore::Semaphore;
use tokio::sync::{mpsc::{self, channel, Receiver, Sender}, RwLock};

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
    rx: Option<Receiver<usize>>,
    priority: Priority,
}

#[derive(Debug, Clone)]
pub enum Priority {
    High = 0, 
    Medium = 1,
    Low = 2,
}

impl TryFrom<usize> for Priority {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Priority::High),
            1 => Ok(Priority::Medium),
            2 => Ok(Priority::Low),
            _ => Err(format!("Invalid value: {}", value)),
        }
    }
}


impl PriorityQueue {
    pub fn new(priority: Priority) -> PriorityQueue {
        Self::with_capacity(32, priority)
    }

    pub fn with_capacity(capacity: usize, priority: Priority) -> PriorityQueue {
        let (tx, rx) = channel::<usize>(capacity);
        PriorityQueue {
            tx,
            rx: Some(rx),
            priority,
        }
    }

    pub async fn push(&mut self, url_idx: usize) {
        let tx_clone = self.tx.clone();
        tokio::spawn(async move {
            tx_clone.send(url_idx).await;
        });
    }

    pub async fn listen_and_notify(&mut self, tx_out: Sender<usize>, notify: Arc<Semaphore<>>) {
        let rx = self.rx.take();
        let tx_out = tx_out.clone();
        let priority = self.priority.clone();
        tokio::spawn(async move {
            if let Some(mut rx) = rx {
                while let Some(url_idx) = rx.recv().await {
                    // Release increments the semaphore count by 1
                    notify.release();
                    tx_out.send(url_idx).await;
                }
            } else {
                panic!("Receiver cannot be acquired in pop_continous()");
            }
        });
    }
}
