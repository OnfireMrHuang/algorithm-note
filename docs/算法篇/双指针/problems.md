# 题解

## 5.最长回文子串

> [题目描述](https://leetcode-cn.com/problems/longest-palindromic-substring/)

**题目解法**: 该题目我们可以拆解为两个过程，一个是判断回文传，一个是如何分解字符串为子串。

- 判断回文串我们可以使用双指针技巧，从字符串的两端向中间遍历，如果最终指针指向的字符都相同，则该字符串为回文串。
- 我们可以使用双循坏暴力遍历字符串，找到所有的子串，然后判断是否为回文串，最终找到最长的回文串。

但是上面的解法时间复杂度比较高，假设两层for循环的时间复杂度是`O(n^2)`,再加上判断回文串的时间复杂度`O(n)`,那么总的时间复杂度就是`O(n^3)`。为了进一步优化，我们可以将双循环的暴力遍历想办法优化为一层循坏，我们假设当前遍历到的字符为`i`,那么我们可以从`i`开始向两边扩散，判断是否为回文串，这样的话时间复杂度就是`O(n^2)`。步骤如下:

- 从左向右遍历字符串，假设当前遍历到的字符为`i`，那么开始求以`i`为中心的最长回文串是多少。
- 以`i`为中心有两种情况:
  - i是奇数子串的中心，那么以`i`为中心的最长回文串就是`i`向两边扩散，直到不是回文串为止。
  - i是偶数子串的中心，那么以`i`为中心的最长回文串就是`i`和`i+1`向两边扩散，直到不是回文串为止。
- 将上面两种情况的最长回文串长度进行比较，取最大值为以`i`为中心的最长回文串。
- 然后不断对比更新每个`i`为中心的最长回文串长度，最终得到最长回文串。

[rust版本](../../../codes/rust/5.最长回文子串.rs) |
[java版本](../../../codes/java/5.最长回文子串.java) |
[golang版本](../../../codes/golang/5.最长回文子串.go) |
[python版本](../../../codes/python/5.最长回文子串.py)

</br>

## 88.合并两个有序数组

> [题目描述](https://leetcode-cn.com/problems/merge-sorted-array/)

</br>

## 26.删除有序数组中的重复项

> [题目描述](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/)

</br>

## 27.移除元素

> [题目描述](https://leetcode-cn.com/problems/remove-element/)


</br>

## 42.接雨水

> [题目描述](https://leetcode-cn.com/problems/trapping-rain-water/)

</br>

## 15.三数之和

> [题目描述](https://leetcode-cn.com/problems/3sum/)

</br>

## 18.四数之和

> [题目描述](https://leetcode-cn.com/problems/4sum/)

</br>

## 234.回文链表

> [题目描述](https://leetcode-cn.com/problems/palindrome-linked-list/)

</br>


## 19.删除链表的倒数第N个节点

> [题目描述](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)

</br>

## 141.环形链表

> [题目描述](https://leetcode-cn.com/problems/linked-list-cycle/)

</br>

## 142.环形链表II

> [题目描述](https://leetcode-cn.com/problems/linked-list-cycle-ii/)

</br>

## 86.分隔链表

> [题目描述](https://leetcode-cn.com/problems/partition-list/)

</br>

## 167.两数之和II-输入有序数组

> [题目描述](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/)

</br>

## 283.移动零

> [题目描述](https://leetcode-cn.com/problems/move-zeroes/)

</br>

## 344.反转字符串

> [题目描述](https://leetcode-cn.com/problems/reverse-string/)

</br>
