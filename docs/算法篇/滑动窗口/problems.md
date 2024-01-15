# 题解

## 3. 无重复字符的最长子串

> [题目描述](https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/)

**题目解法:** 该题目可以通过滑动窗口算法来解决，我们定义从`left`到`right`的区间为滑动窗口，当滑动窗口中的元素不重复时，我们将`right`向右移动，直到滑动窗口中的元素出现重复，此时我们比对最大的非重复子串长度`max_range_length`并更新，接着我们再将`left`向右移动，直到滑动窗口中的元素不重复，然后再将`right`向右移动，重复上述操作，直到`right`到达字符串的末尾。 最后返回`max_range_length`即可。

[rust版本](../../../codes/rust/3.无重复字符的最长子串.rs) |
[java版本](../../../codes/java/3.无重复字符的最长子串.java) |
[golang版本](../../../codes/golang/3.无重复字符的最长子串.go) |
[python](../../../codes/python/3.无重复字符的最长子串.py)

</br>

## 76. 最小覆盖子串

## 239. 滑动窗口最大值

## 438. 找到字符串中所有字母异位词

> [题目描述](https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/)

**题目解法**: 该题目可以先计算出字符串`p`

[rust版本](../../../codes/rust/438.找到字符串中所有字母异位词.rs) |
[java版本](../../../codes/java/438.找到字符串中所有字母异位词.java) |
[golang版本](../../../codes/golang/438.找到字符串中所有字母异位词.go) |
[python](../../../codes/python/438.找到字符串中所有字母异位词.py)

</br>

## 480. 滑动窗口中位数
