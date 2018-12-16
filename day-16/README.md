### Day 16: Chronal Classification

For the first part, just try each operation on samples and record the ones that are valid. Statement is a bit long but implementation is not hard. Actually it takes fewer than 20 lines to implement the execution part.

For the second part, take the intersection of all valid operations for a given op-code, and these should uniquely determine the mapping of op-code to operations. A tricky thing here is that not all intersected sets are of size 1, so you'll need to iteratively determine the mappings: fix mappings for singleton sets, remove the mapped operations from all sets, and repeat until all mappings are determined.