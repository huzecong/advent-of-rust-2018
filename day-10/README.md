### Day 10: The Stars Align

Doing actual OCR is infeasible, so the task for us is to find the time tick when the stars are most likely to form characters, print them and recognize the characters by ourselves.

From observing the examples, we find that the stars are most "tightly-packed" when they form characters. How do we quantify "tight-packed-ness"? The area of their bounding box! We just find the time tick when the stars have the smallest bounding box area.

Turns out this works, and what's more, the bounding box area has a single peak, so you can break when the area goes up.