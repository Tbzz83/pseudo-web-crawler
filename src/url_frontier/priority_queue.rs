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
    pub priority: Priority,
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

    pub async fn pop_left(&mut self) -> Option<usize> {
        if let Some(rx) = &mut self.rx {
            if let Some(url_idx) = rx.recv().await {
                return Some(url_idx);
            }
        }

        None
    }

    pub fn is_empty(&self) -> Option<bool> {
        if let Some(rx) = &self.rx {
            return Some(rx.is_empty());
        }

        None
    }

    pub fn with_capacity(capacity: usize, priority: Priority) -> PriorityQueue {
        let (tx, rx) = channel::<usize>(capacity);
        dbg!(&rx);
        PriorityQueue {
            tx,
            rx: Some(rx),
            priority,
        }
    }

    pub async fn push(&mut self, url_idx: usize) {
        match self.tx.send(url_idx).await {
            Ok(_) => (),
            Err(e) => println!("Error in tx.send(): {:?}", e),
        }
    }

//    pub async fn listen_and_notify(&mut self) {
//        let rx = self.rx.take();
//        let tx_out = self.tx.clone();
//        let priority = self.priority.clone();
//        tokio::spawn(async move {
//            if let Some(mut rx) = rx {
//                while let Some(url_idx) = rx.recv().await {
//                    // Release increments the semaphore count by 1
//                    tx_out.send(url_idx).await;
//                }
//            } else {
//                panic!("Receiver cannot be acquired in pop_continous()");
//            }
//        });
//    }
    
}
