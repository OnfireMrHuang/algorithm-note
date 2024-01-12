# 题解

## 560. 和为K的子数组

[原题链接](https://leetcode-cn.com/problems/subarray-sum-equals-k/)

**题目解法:** 该题目可以通过前缀和 + 哈希表来解决，思路如下:

1. 首先我们维护一个哈希表，其中: <span style="color: brown;">key是前缀和，value是该前缀和对应的下标个数</span>
2. 然后我们遍历数组，每次遍历到一个元素，将前一个元素的前缀和加上当前元素的值，得到当前元素的前缀和，然后将当前元素的前缀和存入哈希表中(对值累计)
3. 接着我们将当前前缀和减去目标值，得到一个值，然后我们去哈希表中查找是否存在这个值，如果存在，那么就说明存在一个子数组的和为目标值，然后我们将这个子数组的个数加入到结果中
4. 最终遍历完数组我们就得到了子数组的个数

[rust版本](../../../codes/rust/560.和为-k-的子数组.rs) |
[java版本](../../../codes/java/560.和为-k-的子数组.java) |
[golang版本](../../../codes/golang/560.和为-k-的子数组.go) |
[python版本](../../../codes/python/560.和为-k-的子数组.py)

## 303. 区域和检索 - 数组不可变

[原题链接](https://leetcode-cn.com/problems/range-sum-query-immutable/)

## 304. 二维区域和检索 - 矩阵不可变

[原题链接](https://leetcode-cn.com/problems/range-sum-query-2d-immutable/)

