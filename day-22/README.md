### Day 22: Mode Maze

The first part just involves some simple calculation. Make sure to store results for each block so you don't compute them repeatedly.

The second part is another shortest path problem. However, the states not only include the coordinates, but also the tool at hand. Also, since transitioning between states have varying costs (1 for moving and 7 for switching tools, both change the current state), we cannot use BFS and have to resort to a proper shortest path algorithm, e.g. Dijkstra's algorithm (optionally heap-optimized).

One tricky point is that the problem allows moving outside of bounds, so we have to determine the board size. We can do this by using a worst-case analysis:

- Suppose we stay in bounds the entire time. For our input, the minimum time is 1032.
- In a perfect world where there's no switching tools, the theoretical lower bound of time is the Manhattan distance between start and target: 709.
- If we walk `k` steps outside the bounds, we would also have to take `k` steps back. For this to be a better plan, we must have `2k < 1032 - 709`, yielding `k <= 161`.
- At the worst, we could expand our board to `(n+k,m+k)`, which for our case is `169 x 862 = 145678`. Multiply that by 3 and we get the total number of nodes in our graph: 437034. This is pretty acceptable for a `O(n log n)` algorithm (that is, suppose you implemented the heap-optimized version).
- But anyway, this is a very pessimistic bound, since it's assuming we don't switch tools at all. In my experiments, decreasing `k` down to 40 still works.
- Last but not least, this is just a post-hoc analysis. During a competition, you would first semi-blindly try a few values and see when it stops decreasing. This would be a good guess.