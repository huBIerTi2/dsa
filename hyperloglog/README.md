
# Table of Contents



HyperLogLog is a cardinality estimator. It lets you calculate the approximate amount of distinct elements in a multiset in linear time with constant memory.

It leverages the heuristic that, <img src="https://render.githubusercontent.com/render/math?math=V=\max_{x \in S} \{ \rho(x) \} \approx |S|"></img>, where <img src="https://render.githubusercontent.com/render/math?math=S"></img> is our set, <img src="https://render.githubusercontent.com/render/math?math=\rho(x)"></img> is the position of the leftmost set bit (counting from the left). It approximates <img src="https://render.githubusercontent.com/render/math?math=V"></img> for multiple subsets of <img src="https://render.githubusercontent.com/render/math?math=S"></img> and calculates the average.

This method gives around 2% of error with \`2kB\`(!) of data needed. (For <img src="https://render.githubusercontent.com/render/math?math=|S| < 10^9"></img>)

