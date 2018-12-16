###Day 3: No Matter How You Slice It

The most straightforward approach is to create a 2D array, with each element representing the number of "claims" (rectangles) covering the square.

To compute the values of the 2D array, the brute-force algorithm is to add 1 to each square inside each rectangle. For $n$ rectangles on a $w\times h$ canvas, the time complexity is $O(nwh)$. This is acceptable given constraints of this problem, but could be made much faster.

A smarter approach would be to use [2D prefix sums](https://www.geeksforgeeks.org/prefix-sum-2d-array/). This reduces complexity to $O(n+wh)$.

For part 2, compute the prefix sum again so we can quickly compute the total number of claims for each square in a rectangle. Find the rectangle such that its area is equal to the its computed total (i.e. each square is covered once).