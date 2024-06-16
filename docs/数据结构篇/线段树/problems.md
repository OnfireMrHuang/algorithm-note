# 题解

## 307. 区域和检索 - 数组可修改

> [题目描述](https://leetcode-cn.com/problems/range-sum-query-mutable/)

**题目解法:** 该题是典型的线段树解法，可参考简要说明中的线段树操作一节。

[rust版本](../../../codes/rust/307.区域和检索-数组可修改.rs) |
[java版本](../../../codes/java/307.区域和检索-数组可修改.java) |
[golang版本](../../../codes/golang/307.区域和检索-数组可修改.go) |
[python版本](../../../codes/python/307.区域和检索-数组可修改.py)

## 729. 我的日程安排表 I

> [题目描述](https://leetcode-cn.com/problems/my-calendar-i/)

**题目解法:** 该题目的要求很简单，就是预定的时候判断**预定区间**与**已定区间**是否存在冲突，如果没有，则加入并和**已定区间**合并融合。如果有冲突，提示预定失败。
现在要考虑的就是使用什么数据结构来存储已定区间， 这里我们知道区间之间是可以顺序的(数组或链表性质)，同时当插入一个新区间时需要快速判断是否有冲突；而线段树正好满足这个需求。

另外，在之前的线段树定义中，叶子节点存储的是`start==end的区间`, 这里则存储的则是`已定区间`; 仔细一想，这里其实就是一个存储区间值的B-tree定义.

[rust版本](../../../codes/rust/729.我的日程安排表-i.rs) |
[java版本](../../../codes/java/729.我的日程安排表-i.java) |
[golang版本](../../../codes/golang/729.我的日程安排表-i.go) |
[python版本](../../../codes/python/729.我的日程安排表-i.py)

## 731. 我的日程安排表 II

> [题目描述](https://leetcode-cn.com/problems/my-calendar-ii/)

**题目解法:** todo

[rust版本](../../../codes/rust/731.我的日程安排表-ii.rs) |
[java版本](../../../codes/java/731.我的日程安排表-ii.java) |
[golang版本](../../../codes/golang/731.我的日程安排表-ii.go) |
[python版本](../../../codes/python/731.我的日程安排表-ii.py)

## 732. 我的日程安排表 III

> [题目描述](https://leetcode-cn.com/problems/my-calendar-iii/)

**题目解法:** todo

[rust版本](../../../codes/rust/732.我的日程安排表-iii.rs) |
[java版本](../../../codes/java/732.我的日程安排表-iii.java) |
[golang版本](../../../codes/golang/732.我的日程安排表-iii.go) |
[python版本](../../../codes/python/732.我的日程安排表-iii.py)
