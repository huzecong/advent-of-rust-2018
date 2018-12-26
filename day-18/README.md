### Day 18: Settlers of The North Pole

The first part is pretty easy, just simulate.

For the second part, simulating for that many rounds is not possible. Note that the rules here is pretty similar to the [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life), and we know that it has cycles, i.e. after a certain number of rounds the entire board would be identical to some previous point. If we have a cycle starting at round `l` and ending at round `r`, the board at round `x` would be identical to round `(x-l) mod (r-l)`.

For detecting cycles, I computed the hash of the board and stored it in a dictionary. The board can be represented as a ternary number (with each square type being digits 0 to 2), so hashing is easy. I used two hashes to be safe, but just using one should also do the job.