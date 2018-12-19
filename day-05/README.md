### Day 5: Alchemical Reduction

Finally one that involves a bit of algorithms. The two parts are basically the same, while the second just requires your code to run faster.

When two units react, they create an opportunity for the units adjacent to them to react. Besides this, the reaction creates no impacts. This means that reactions could happen in any order, and any order would lead to the optimal solution.

Given this observation, a naïve approach is to iteratively scan the string from left to right, removing every pair that could react along the way. In the worst case, for a string of length $n$, $O(n)$ passes are required, yielding time complexity of $O(n^2)$. It takes about 5 seconds to run the first task on my computer, and should work for the second task, but we could do it faster.

In the faster algorithm, we still scan from left to right, but now we keep two pointers that point to the potential left and right units. When they react, we move the left pointer to the left, and the right pointer to the right, and check whether these adjacent units can react. If they can, repeatedly react until the adjacent units cannot, or we have reached the boundaries. Then, move both pointers to the right of the current right pointer, and continue scanning.

To see why this is constant, let's check how many times each pointer would sweep across each unit. The right pointer is monotonic, so it sweeps each unit exactly once. For the left pointer, the number of times it moves towards the left is equal to the number of pairs we find react. Since this is less than string length, the left pointer sweeps each unit no more than twice.

The efficient implementation requires a data structure that supports finding the next and previous elements in a sequence, and removing elements from the middle. A doubly-linked list would be perfect for the task. However, implementing a linked list in Rust is notoriously hard, so I kept track of a `previous` array that stores indices of the previous element in the list. See my code for details.