HyperLogLog is a cardinality estimator. It lets you calculate the
approximate amount of distinct elements in a multiset in linear time
with constant memory.

It leverages the heuristic that,
$V=\max_{x \in S} \{ \rho(x) \} \approx |S|$, where $S$ is our set,
$\rho$ is the position of the leftmost set bit (counting from the left).
It approximates $V$ for multiple subsets of $S$ and calculates the
average.

This method gives around 2% of error with \`2kB\`(!) of data needed.
(For $|S| < 10^9$)
