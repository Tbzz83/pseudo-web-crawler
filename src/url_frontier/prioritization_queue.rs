/// NOTE
/// The plan for this is that this struct will have a vec deque. 
/// BUT, crucially it also contains some metadata about the queue, 
/// such as how many high prioritization urls it contains. Or we're 
/// going to have to have some value/weight that indicates its priority. 
/// The prioritizer is going to basically loop through all our PrioritizationQueue 
/// structs, and push_front high priority urls to queues with the LOWEST (configurable)
/// priority, to keep things fair
