### Day 7: The Sum of Its Parts

The first part is finding the [topological ordering](https://en.wikipedia.org/wiki/Topological_sorting) of a directed acyclic graph. There's a well-known algorithm for this that runs in linear time. To better describe the second part solution, I'll have to briefly go over the algorithm (note that I use the terms "job" and "node" interchangeably):

1. Compute the "in-degree" (i.e. the number of edges pointing to a node) of each node.
2. Maintain a queue. Push all nodes with in-degree of 0 into the queue.
3. Pop a node from the queue. For each edge pointing out from the node, decrement the in-degree of the target node. If the in-degree drops to 0, push it into the queue.
4. While the queue is not empty, go to step 3.

But the requirements here are a bit different: always choose the smallest node. We can change the queue to a priority queue (heap), or simply scan through all nodes and pick the smallest.

For the second part, we have to maintain another queue (job queue) of in-progress jobs. We make a slight modification to the algorithm above:

- While we have free workers and the node queue is non-empty, pop a node from the queue and put a worker on the job. We do not decrement in-degrees at this point.
- When we either run out of workers or have no jobs to do, pop a job from the job queue. This means we've done the job, so we keep track of the current time, and decrement the in-degrees.
- Repeat the above steps until everything is done.