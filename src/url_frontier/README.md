# Url_frontier Module

### UrlFrontier

- Holds AllUrls, PriorityQueues, DomainQueues
- Is the main orchestrator of application logic

### AllUrls

- SoA object that stores data for all of our URLs
- Something can access a url via a url_idx
- Private fields
- Has a `compose_url_from_idx()` function that returns `Option<Url>` based on url_idx
- Delete functionality:
  - When a Url is deleted, the data still exists in AllUrls, but the free_slots field acquires its index. This way, we know if it can be used again.
  - When a new Url is added, we first check if there are any free idx in free_slots ? Use that and overwrite data in that idx : push new url

### Url

- AoS singular for AllUrls

### PriorityQueue

- Is a queue of url_idx values
- Stores some additional metadata that is used to calculate the priority of that queue, based on the url domain name
- PriorityQueues are automatically populated eagerly when a new Url is added.
- A PriorityQueue can be removed, and must be removed in order for data to move. It is replaced and the whole queue itself is sent to the BackQueueRouter. In this way, no two queues can ever hold the same url_idx, it will only ever be held by one queue.

### BackQueueRouter *(static (I think? Holds no data, just does processing))*

- Input is a PriorityQueue.
- Processes the PriorityQueue and sorts url_idx into specific DomainQueues based on the url domain name. Url_idx values are removed from the PriorityQueue when they are done this way, so by the end the PriorityQueue will be completely consumed and can be dropped.
- The output from the BackQueueRouter is a DomainQueue, and is sent back into the UrlFrontier.

### DomainQueue

- List of queues that contains url_idx values
- Each domain queue only contains Urls that have the same domain name
- This queue will be consumed when Urls are about to be downloaded. A Url object will be constructed from url_idx values, and the queue will be popped from. Individual Url objects will then be sent to the html_downloader module worker.
  - At this point, the url_idx should be deleted from AllUrls by the UrlFrontier orchestrator.

---

### Issues

- Potential issue:
  - We should be aware that a url_idx could be deleted by anything owning the AllUrls type (UrlFrontier). 

### To-do

- Async
  - Queues to channels:
    - Currently, all the queues are literally queues, but this whole application should really be async, and these queues should really be channels.
    - A url should be sent through the PriorityChannel. It's also a challenge, because Our PriorityChannel still needs metadata about the priority, so as a Url leaves the channel, we have to eagerly update its weight.
    - can use tokio::mpsc multi-producer single-consumer channels
