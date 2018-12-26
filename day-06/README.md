### Day 6: Chronal Coordinates

Nice problem. Pretty algorithmic too.

For the first part, every coordinate that belongs to a finite area must be within bounds set by the points (i.e. `minX <= X <= maxX`, `minY <= Y <= maxY`). The area is not large so we simply compute the distances and check.

To determine which areas are infinite, we can find out the belonging areas for each coordinate on the border. Their belonging areas are the infinite ones. Necessity is obvious, sufficiency can be intuitively proved by observing: for a point on the upper border, moving it further up increases its distances to every point by the same amount, so the area it belongs stays the same.

For the second part, we can't enumerate all points because they could be many points outside the bounds. However, we can utilize the fact that the x-axis and y-axis components of Manhattan distance are independent. This reduces the 2D problem into two 1D problems: given $n$ points on a straight line, find the number of points with sum of distance equal to $v$. Denote the x-axis count as ![c_x(v)](http://quicklatex.com/cache3/9c/ql_e5503ccf81e970060a956dcf53b6f79c_l3.png), y-axis as ![c_y(v)](http://quicklatex.com/cache3/cb/ql_64ff8852e2afe643ca891fc202fa28cb_l3.png), the answer would be ![\sum_{x=0}^{10000}\sum_{y=0}^{10000-x}c_x(x)c_y(y)](http://quicklatex.com/cache3/21/ql_357db342f5beadb8636f10aa112d1f21_l3.png).

To solve the 1D case, we notice that the valid coordinate cannot be more than `10000/n` to the left of the leftmost point, nor more than `10000/n` to the right of the rightmost one. So we can enumerate the points and compute distances. I used some other approach that does not require repeatedly computing sum of distances, but rather processes all points in an order that allows me to cheaply compute changes in the sum. See the code for details.