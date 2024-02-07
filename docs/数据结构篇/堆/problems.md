# 题解

## 215. 数组中的第K个最大元素

> 题目描述: https://leetcode-cn.com/problems/kth-largest-element-in-an-array/

**题目解法**: 该题目正好可以使用小根堆；我们可以维护一个k大小的小根堆，让插入一个数值导致堆满的时候就将堆顶元素删除(删除的是最小的元素)，那么当nums的所有元素都走过一遍后，堆顶元素就是第K大的元素。整体的时间复杂度是`O(nlogn)`.

[rust版本](../../../codes/rust/215.数组中的第k个最大元素.rs) |
[java版本](../../../codes/java/215.数组中的第k个最大元素.java) |
[golang版本](../../../codes/golang/215.数组中的第k个最大元素.go) |
[python版本](../../../codes/python/215.数组中的第k个最大元素.py)

</br>

## 295. 数据流的中位数

> 题目描述: https://leetcode-cn.com/problems/find-median-from-data-stream/

**题目解法**: 该题目计算数据流中的中位数，我们可以利用双堆技巧。即使用一个大根堆和一个小根堆，其中大根堆存储元素较小的一半元素(命名为left_heap)，小根堆存储元素较大的一半元素(命名为right_heap)，最后结果可以分别从两个堆的堆顶元素计算得出。整个过程如下:

- MedianFinder类维护left_heap和right_heap
- 当插入元素时，对比一下left_heap和right_heap的大小来保持平衡
- 如果两个堆大小一致，即偶数: 那么就先往right_heap插入一个数据，然后再弹出right_heap的堆顶元素到left_heap,此时left_heap的堆顶元素就是中位数。
- 如果两个堆大小不一致，即奇数: 那么就先往left_heap插入一个数据，然后再弹出left_heap的堆顶元素到right_heap,此时`(left_heap的堆顶元素 + right_heap的堆顶元素) / 2`就是中位数。
- 当数据流写完，按照两个堆的大小和堆顶元素计算中位数即可。

**整个过程中说明一下，为什么要先插入一个堆然后再弹出堆顶元素到另外一个堆？(我当时也有类似的疑惑)** 

因为我们要保持left_heap的堆顶元素始终是中位数，所以我们要保证left_heap的堆顶元素始终是right_heap的堆顶元素的前一个元素，反之亦然。如果不这样做，而是谁少补谁的话，那么两个堆顶元素不能保证是连续的。

[rust版本](../../../codes/rust/295.数据流的中位数.rs) |
[java版本](../../../codes/java/295.数据流的中位数.java) |
[golang版本](../../../codes/golang/295.数据流的中位数.go) |
[python版本](../../../codes/python/295.数据流的中位数.py)

</br>

## 347.前k个高频元素

> 题目描述: https://leetcode-cn.com/problems/top-k-frequent-elements/

**题目解法**: 该题目是计算频率最高的前k个元素，我们可以使用小根堆来解决。整体的思路是:

- 首先我们遍历一遍数组，统计每个元素的频率，形成一个新的`数值-频率数组`
- 维护一个大小为K的小根堆, 将`数值-频率`数组的前K个值作为初始化元素
- 然后继续遍历`数值-频率`数组剩下的元素，对比元素的频率和堆顶元素的频率，如果元素的频率大于堆顶元素的频率，那么就将堆顶元素弹出，然后将元素插入堆中，否则跳过
- 最后只需要遍历小根堆中的元素，将元素中的数值追加到结果即可

[rust版本](../../../codes/rust/347.前-k-个高频元素.rs) |
[java版本](../../../codes/java/347.前-k-个高频元素.java) |
[golang版本](../../../codes/golang/347.前-k-个高频元素.go) |
[python版本](../../../codes/python/347.前-k-个高频元素.py)

</br>

## 378.有序矩阵中第K小的元素

> 题目描述: https://leetcode-cn.com/problems/kth-smallest-element-in-a-sorted-matrix/

**题目解法**: 该题目我们可以通过维护一个大小为K大根堆来解决，我们顺序遍历数组，将前K个元素初始化到大根堆中，然后将剩余的元素写入大根堆的同时也将大根堆的堆顶元素弹出，最后大根堆顶堆顶元素就是第K小的元素.

其中元素的顺序通过`i * n + j`来计算，其中`i`是行，`j`是列，`n`是矩阵的列数。

[rust版本](../../../codes/rust/378.有序矩阵中第-k-小的元素.rs) |
[java版本](../../../codes/java/378.有序矩阵中第-k-小的元素.java) |
[golang版本](../../../codes/golang/378.有序矩阵中第-k-小的元素.go) |
[python版本](../../../codes/python/378.有序矩阵中第-k-小的元素.py)

</br>

## 407.接雨水II

> 题目描述: https://leetcode-cn.com/problems/trapping-rain-water-ii/

**题目解法**: todo

[rust版本](../../../codes/rust/407.接雨水-ii.rs) |
[java版本](../../../codes/java/407.接雨水-ii.java) |
[golang版本](../../../codes/golang/407.接雨水-ii.go) |
[python版本](../../../codes/python/407.接雨水-ii.py)

</br>

## 692.前K个高频单词

> 题目描述: https://leetcode-cn.com/problems/top-k-frequent-words/

**题目解法**: todo

[rust版本](../../../codes/rust/692.前k个高频单词.rs) |
[java版本](../../../codes/java/692.前k个高频单词.java) |
[golang版本](../../../codes/golang/692.前k个高频单词.go) |
[python版本](../../../codes/python/692.前k个高频单词.py)
