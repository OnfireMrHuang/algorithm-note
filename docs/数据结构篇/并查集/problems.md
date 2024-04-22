# 题解

## 547.省份数量

> [题目描述](https://leetcode-cn.com/problems/number-of-provinces/)

**题目解法:** 该题目对省份的定义是包括一组直接相连或间接相连的城市，如果两个城市没有任何直接相连或间接相连的关系，那么就是独立的省份。这个问题可以使用并查集来解决：

1. 一开始假设每个城市都是一个独立的省份，所以有`n`个省份。
2. 从第一个城市开始，遍历后面其他所有城市，如果直接相连，那么将两个城市所在的省份合并。
3. 从第二个城市开始，遍历后面其他所有城市(与第一个城市已经在上一步判断了)，将直接关联的城市合并
4. 重复以上步骤，遍历完所有城市，此时，所有直接或间接相连的城市都已经合并到一个省份中(并查集)。
5. 最后有多少个集合就有多少个省份.

[rust版本](../../../codes/rust/547.省份数量.rs) |
[java版本](../../../codes/java/547.省份数量.java) |
[golang版本](../../../codes/golang/547.省份数量.go) |
[python版本](../../../codes/python/547.省份数量.py)

## 684. 冗余连接

> [题目描述](https://leetcode-cn.com/problems/redundant-connection/)

**题目解法:** 该题目是经典的判断成环问题，可以使用并查集来解决，解决思路是:

1. 通过根节点来判断是否在同一个集合
2. 遍历edg的两个节点，判断两个节点是否为同一个集合，如果已经是，那么说明这条表构成环了，如果不是，那么就加入到集合中，然后继续遍历剩余的edg

[rust版本](../../../codes/rust/684.冗余连接.rs) |
[java版本](../../../codes/java/684.冗余连接.java) |
[golang版本](../../../codes/golang/684.冗余连接.go) |
[python版本](../../../codes/python/684.冗余连接.py)

## 685. 冗余连接 II

> [题目描述](https://leetcode-cn.com/problems/redundant-connection-ii/)

**题目解法:** 该题目相对`冗余连接I`增加了一个条件，即一个节点只能有一个父节点，这样就会出现两种情况:

1. 一个节点存在两个父节点
2. 每个节点都只有一个父节点，但是构成了环

于是我们需要分为两种情况来处理:

1. 首先检查有向图中的顶点是否存在入度为2的情况，如果存在，那么删除其中一条，然后判断有向图是否为有根树，如果是，那么就是删除的这条边，如果不是那就是另外一条
2. 在排除了入度为2的情况下，检测有向图是否有环，如果有环，那么就是构成环的这条边

[rust版本](../../../codes/rust/685.冗余连接-ii.rs) |
[java版本](../../../codes/java/685.冗余连接-ii.java) |
[golang版本](../../../codes/golang/685.冗余连接-ii.go) |
[python版本](../../../codes/python/685.冗余连接-ii.py)

## 990.等式方程的可满足性

> [题目描述](https://leetcode-cn.com/problems/satisfiability-of-equality-equations/)

**题目解法:** todo

[rust版本](../../../codes/rust/990.等式方程的可满足性.rs) |
[java版本](../../../codes/java/990.等式方程的可满足性.java) |
[golang版本](../../../codes/golang/990.等式方程的可满足性.go) |
[python版本](../../../codes/python/990.等式方程的可满足性.py)

## 1319.连通网络的操作次数

> [题目描述](https://leetcode-cn.com/problems/number-of-operations-to-make-network-connected/)

[rust版本](../../../codes/rust/1319.连通网络的操作次数.rs) |
[java版本](../../../codes/java/1319.连通网络的操作次数.java) |
[golang版本](../../../codes/golang/1319.连通网络的操作次数.go) |
[python版本](../../../codes/python/1319.连通网络的操作次数.py)
