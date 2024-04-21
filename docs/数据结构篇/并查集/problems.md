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

**题目解法:** todo

[rust版本](../../../codes/rust/684.冗余连接.rs) |
[java版本](../../../codes/java/684.冗余连接.java) |
[golang版本](../../../codes/golang/684.冗余连接.go) |
[python版本](../../../codes/python/684.冗余连接.py)

## 685. 冗余连接 II

> [题目描述](https://leetcode-cn.com/problems/redundant-connection-ii/)

**题目解法:** todo

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
