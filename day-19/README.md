### Day 19: Go With The Flow

The first part is easy: just modify code from day 16 and run.

The second part is tricky. At first I thought I could just run the interpreter again, but no results came out after 5 minutes. I realized that we had to analyze the Elf code, but couldn't bring myself to it. In the end, I gave up on the second star.

[This reddit thread](https://www.reddit.com/r/adventofcode/comments/a7j9zc/2018_day_19_solutions/ec3i5og) describes what the Elf code does pretty well. It's basically a brute-force integer factorization algorithm that calculates the sum of divisors for a large number generated in the code.