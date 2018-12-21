### Day 21: Chronal Conversion

This is pretty much the same as day 19, you're given elf code and you need to analyze it. In hindsight, it is unnecessary to spend so much time analyzing the code, simply going through it and writing a translator would suffice. Anyway I'll go through the code here.

First thing to note is that we're only allowed to change register 0, and the only line which has something to do with r0 is line 28, where r0 is checked to see if it is equal to r4. If equality is satisfied, the program ends, otherwise it loops.

So the first part is pretty easy, just print the value of r4 when the program first hits line 28. You don't even need a translator, our interpreter from day 19 could spit the result in less than 3 seconds.

The second part is similar. If you dig a bit more into the code, you'll realize that only r3 and r4 are actually useful, the rest are all auxiliary variables. Since the code is deterministic (no random stuff etc.), r3 and r4 represent the "state" of the simulated device: given values for r3 and r4, we can uniquely determine the next pair of values for them. What this means is that, the program is bound to loop forever: it will eventually land in a state it has been in before, and thus entering a loop. The answer for the second part will be the value of r4 before it has entered the loop, i.e. the previous value of r4 when we first encounter a value of r4 that we've seen before. Just use a set and a list to keep track of history.

If we do more analysis on the code, we would eventually get to the Python code in `main.py`, where we simplified a bunch of loops. The loop between lines 17-27 is actually trying to find the least multiple of 256 that's greater than r3, and storing the multiplied coefficient minus 1. This is the most time consuming part and could be replaced with a simple integer division. But anyway, the translated program could be fast enough.