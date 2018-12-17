### Day 17: Reservoir Research

This one is pretty tricky. To simulate you have to understand the water dynamics in this problem, and I'll summarize here:

- Obviously, water falls down (direction of increasing Y), starting from the spring at (500, 0).
- When water reaches a "supported" square (i.e. a square with clay `#` or previously filled still water `~` as the lower square), it fills the square, and spreads to the furthest left and right squares, until it either reaches clay or an unsupported square.
  - If an unsupported square is reached, the previous steps are recursively executed. After that, if the square becomes supported (i.e. water filled the entire bottom), then continue spreading.
  - If eventually the left- and rightmost squares are both supported and blocked by clay, we call the row "filled". In this case, this row is "still water" `~`, it would remain in place if the spring stopped flowing. Otherwise, the row is "flowing water" `|`.
- When water falls beyond the maximum Y line, everything beyond will be flowing water and we stop simulation.

This is clearly a recursive solution, so I implemented it as a recursive function `flood(x, y, direction) -> bool`, which given the current coordinates and the direction of the water flow, return whether this block is eventually supported and blocked by clay. `direction` indicates where the water flows from, and can be `UP`, `LEFT`, or `RIGHT`. This is required to prevent infinite recursion by expanding only in the opposite direction (in case of `UP`, also expand to the sides if supported).

After the simulation is complete, simply count how many `~`s and `|`s are on the map. Answer to the first part is the sum of two counts, and that to the second part is the count of `~`s.

**Note:** There are a few tricky spots in the implementation, all of them could be covered with this wonderful example on [this subreddit thread](https://www.reddit.com/r/adventofcode/comments/a6wpup/2018_day_17_solutions/ebyws5k):

```
y=1, x=499..501
y=5, x=496..504
x=496, y=4..5
x=504, y=3..5
```

and the solution should be:

```
...||+||..
...|###|..
...|...|..
|||||||||#
|#~~~~~~~#
|#########
```

**PS:** A bug I had is that I only counted the water of the columns between the minimum and maximum X's. Somehow the answer I had in this case was the answer of some other input, and AoC thought I was cheating :(