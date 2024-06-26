# 题解

## 200. 岛屿数量

> [题目描述](https://leetcode-cn.com/problems/number-of-islands/)

**题目解法:** 这道题目中关于岛屿的定义是对于一个连通的`1`组成的区域(水平加垂直)，如果其水平和垂直方向上都是0的情况下，那么就是岛屿。这道题目我们可以使用深度优先遍历来解决，我们只需要从网格值为`1`的陆地开始往四个方向深度遍历，遇到陆地就组合在一起(标记为已访问)，遇到网格值为`0`的水就停止，这样就找到了第一个岛屿，接下来再从未访问过的陆地开始深度遍历，重复以上过程，最终计算出岛屿数量。

tips: 我们可以在深度遍历过程中将遇到的陆地直接淹掉(置为0)，这样可以省略对`visited`的维护.

[rust版本](../../../codes/rust/200.岛屿数量.rs) |
[java版本](../../../codes/java/200.岛屿数量.java) |
[golang版本](../../../codes/golang/200.岛屿数量.go) |
[python版本](../../../codes/python/200.岛屿数量.py)

## 207. 课程表

> [题目描述](https://leetcode-cn.com/problems/course-schedule/)

**题目解法:** 该题目的问题是 "请你判断是否可能完成所有课程的学习？" 那么什么情况下不能完成呢，当课程之间存在环路的时候，是无法完成所有课程的学习的。所以我们可以将该问题转化为有向图中是否存在环的问题，如果存在环，那么就无法完成所有课程的学习。

于是我们需要分为一下几步:

1. 基于`prerequisites`构建有向图
2. 从[0..numCourses]中顺序选择节点作为在图中的起始节点，然后使用深度优先遍历的方式遍历整个图，如果遇到已经访问过的节点，那么就说明存在环，无法完成所有课程的学习。

[rust版本](../../../codes/rust/207.课程表.rs) |
[java版本](../../../codes/java/207.课程表.java) |
[golang版本](../../../codes/golang/207.课程表.go) |
[python版本](../../../codes/python/207.课程表.py)

## 210. 课程表 II

> [题目描述](https://leetcode-cn.com/problems/course-schedule-ii/)

**题目解法:** 课程表II要求返回学完所有课程的任意一个顺序，该题目可以使用拓扑排序来解决，也可以沿用207题的路径来解决，我们这里选择使用拓扑排序的方式来实现:

1. 同样的先基于`prerequisites`构造有向图
2. 构造每个课程的入度数组`indegree`，入度为0的课程就是可以学习的课程
3. 选择一个入度为0的顶点并加入到结果中
4. 然后删除该顶点的所有出边，更新入度数组，如果有新的入度为0的顶点，那么就加入到结果中
5. 重复以上过程，直到所有的顶点都被加入到结果中；如果存在有顶点的入度不为0的，那么说明图中存在环

[rust版本](../../../codes/rust/210.课程表-ii.rs) |
[java版本](../../../codes/java/210.课程表-ii.java) |
[golang版本](../../../codes/golang/210.课程表-ii.go) |
[python版本](../../../codes/python/210.课程表-ii.py)

## 743. 网络延迟时间

> [题目描述](https://leetcode-cn.com/problems/network-delay-time/)

**题目解法:** 该题目是一个有向图中求最短路径的问题，我们可以把延迟看作边的权重，最小延迟就是最小权重路径，于是我们可以使用Dijkstra算法来解决。

`Dijkstra`本质上是一个贪心算法，通过每次从最短路径的顶点开始，逐渐扩展到其他顶点，并且每个顶点都只记录起点到该顶点的目前为止的最短路径，直到所有顶点都被遍历到，最终某个顶点的路径是最长的，那么这个路径就是从起点出发广播完所有节点所需要的最短路径。

[rust版本](../../../codes/rust/743.网络延迟时间.rs) |
[java版本](../../../codes/java/743.网络延迟时间.java) |
[golang版本](../../../codes/golang/743.网络延迟时间.go) |
[python版本](../../../codes/python/743.网络延迟时间.py)

## 1584. 连接所有点的最小费用

> [题目描述](https://leetcode-cn.com/problems/min-cost-to-connect-all-points/)

**题目解法:** 任意两点之间`有且仅有`一条简单路径，才认为所有点都已连接，而点与点之间的边无向，说明这是无向图，并且点之间不构成环，是一个最小生成树问题（即连接所有点的最小费用就是最小生成树的权重和），我们使用`Kruskal`算法来解决该问题。

`kruskal`本质上是一个贪心算法，通过每次从无向图中选择权重最小的边，判断该边与已存在的并查集是否构成环，如果不构成环，那么就将该边加入到最小生成树中，直到所有的边都被遍历到，最终得到最小生成树。

[rust版本](../../../codes/rust/1584.连接所有点的最小费用.rs) |
[java版本](../../../codes/java/1584.连接所有点的最小费用.java) |
[golang版本](../../../codes/golang/1584.连接所有点的最小费用.go) |
[python版本](../../../codes/python/1584.连接所有点的最小费用.py)
