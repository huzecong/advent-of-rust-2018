### Day 12: Subterranean Sustainability

Part 1 is also a simulation problem. One tricky thing to think about is: what is the minimum (and maximum) pot number during the first 20 rounds? By analyzing the input, we can know that pots extend to either side by no more than one per round.

> This is because we have rules `...## => #` and `.#... => #` which would extend one pot to either side. But we don't have rules like `....# => #` or `#.... => #` so pots wouldn't extend to two positions away.

Part 2 is intractable, so there must be a pattern. In fact, if you run the simulation long enough (~120 rounds), you can observe that the pattern becomes `.....#.#.#.#.#.....` (actual sequence is much longer), and the sequence shifts one position to the right at every round. So we don't have to simulate the remaining billions of rounds, just compute the positions based on the pattern.

But of course, this is what happens in my input, but I'm sure similar patterns exist across different inputs.