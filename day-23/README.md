### Day 23: Experimental Emergency Teleportation

Okay this one's tough. Lot's of people have solved part 2, but AFAIK no solution is either robust or has a provably polynomial complexity. Such solutions include:

- "Space partitioning" approaches that iteratively halves the search space, and eventually narrowing down to a single point. [This thread](https://www.reddit.com/r/adventofcode/comments/a9co1u/day_23_part_2_adversarial_input_for_recursive/) contains an adversarial input to these solutions.
- The above approach combined with data structures or heuristics and sorts. Among them a nice one is describe in [this thread](https://www.reddit.com/r/adventofcode/comments/a99n7x/2018_day_23_part_2_explanation_with_visualization/), but unfortunately is still not provably fast.
- [Simulated annealing](https://en.wikipedia.org/wiki/Simulated_annealing): taking steps with iteratively decreasing lengths in each direction, and making a move when it yields a better solution. This has no guarantee of finding a global maximum.
- Treating each nanobot as a constraint and perform optimization using an extremely strong library. Most people used the [Z3 Satisfiability Modulo Theories Solver](https://github.com/Z3Prover/z3) in Python. I am not sure what it does, but to me it seems like solving either an [n-SAT problem](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem) or an [integer programming problem](https://en.wikipedia.org/wiki/Integer_programming), both of which are NP-Complete.
- [Maximum clique](https://en.wikipedia.org/wiki/Clique_problem) approaches that assumes the best solution is at the intersection of a set of nanobots that is in range of each other, which corresponds to a clique. This assumption does not always hold true, and the maximum clique problem is also NP-Complete.

A lot of discussing has being going on inside [this thread](https://www.reddit.com/r/adventofcode/comments/a8sqov/help_day_23_part_2_any_provably_correct_fast/), but unfortunately as of the time of writing there are no satisfiable solutions.

I have discussed with people and was also unable to come up with a solution that satisfies the criteria. So I'm temporarily giving up on this problem.