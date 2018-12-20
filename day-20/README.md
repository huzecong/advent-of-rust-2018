### Day 20: A Regular Map

The two parts are similar, and the difficulty is in generating the map.

When we scan the regex string, we have to keep track of two things: the set of possible current positions, and the set of possible ending positions at this "level". By level I mean the longest subsequence wrapped in brackets that contains the current character:

```
NNN(NNNN(NN|N)|NN)NN
   ~~~~^~~~~~~~~~~
    current level
```

The reason we need to keep track of the two sets are:

- It's possible that paths have diverged **before** the current character, and we may end up at different positions, so we keep the current positions set.
- It's possible that we're currently in a branch, and we only need the ending positions from each branch (so we could merge them into the current positions set of the previous level). That's why we need the ending positions set.

We also need a stack to store the sets for each level. Parsing this string can be a bit tricky so you'll have to think thoroughly.

Although, as discussed in [this reddit thread](https://www.reddit.com/r/adventofcode/comments/a7w4dj/2018_day_20_why_does_this_work/), there are special properties about the data which could simplify things (I guess that's intentional to make the [code golf competition](https://www.reddit.com/r/adventofcode/comments/a7vu8h/advent_of_golf_is_today/) more fun).