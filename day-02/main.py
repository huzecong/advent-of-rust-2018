from collections import Counter
import itertools

with open('input.txt') as f:
    strs = f.read().split()

# Part 1
twos = sum(int(any(v == 2 for v in Counter(s).values())) for s in strs)
threes = sum(int(any(v == 3 for v in Counter(s).values())) for s in strs)
print(twos * threes)

# Part 2
for sa, sb in itertools.combinations(strs, 2):
    common = [a for a, b in zip(sa, sb) if a == b]
    if len(common) == len(sa) - 1:
        print(''.join(common))
        break
