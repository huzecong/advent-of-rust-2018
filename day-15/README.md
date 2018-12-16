###Day 15: Beverage Bandits

A simulation problem. Don't get frightened of the length of the statement, just do as it says.

To find the nearest squares, simply run the floodfill (BFS) algorithm. To find the next step to take, either record back-pointers during floodfill or do another floodfill starting from the target square.

A tricky point: There's a difference between walking towards the nearest **target** and towards the nearest **square in range**. Consider this map:

```
######
##G..#
#..#.#
#.##.#
#.!E!#
######
```

For the goblin, taking either right or down would lead to the elf in the same number of steps, and by reading order taking right is preferred. However, in terms of the squares in range (squares marked with `!`), by reading order the left one is preferred, which means the goblin should take down instead of right.

Being incorrect on this detail did not prevent me from passing part one, but got me stuck on part two for over an hour.