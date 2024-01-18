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

> [题目描述](https://leetcode-cn.com/problems/minimum-window-substring/)

**题目解法:** 该题目存在一个主串、一个模式串，要求在主串中找到一个最小的能够覆盖模式串中所有字符的子串。同样我们可以通过滑动窗口来解决，我们定义`left`和`right`分别对应滑动窗口的左右边界，当滑动窗口没有覆盖模式串中所有字符时，我们移动`right`指针，直到滑动窗口覆盖了模式串中所有字符，此时对比最小覆盖子串长度`min_sub_string_len`来更新最小覆盖子串，接着再移动`left`指针，直到滑动窗口不再覆盖模式串中所有字符，然后再移动`right`指针，重复上述操作，直到`right`到达字符串的末尾。 最后返回最小覆盖子串即可。

判断滑动窗口是否满足覆盖模式串:

- 给模式串维护一个哈希表，其中key是字符，value时字符出现的次数
- 给滑动窗口维护一个哈希表，其中key是字符，value是字符出现的次数
- 每次滑动窗口的时候通过对比模式串哈希表来查询并更新`valid_num`值来和模式串长度进行比较，如果`valid_num`等于模式串长度，则说明滑动窗口已经覆盖了模式串中所有字符

[rust版本](../../../codes/rust/76.最小覆盖子串.rs) |
[java版本](../../../codes/java/76.最小覆盖子串.java) |
[golang版本](../../../codes/golang/76.最小覆盖子串.go) |
[python](../../../codes/python/76.最小覆盖子串.py)

</br>

## 438. 找到字符串中所有字母异位词

> [题目描述](https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/)

**题目解法**: 该题目可以先计算出字符串`p`

[rust版本](../../../codes/rust/438.找到字符串中所有字母异位词.rs) |
[java版本](../../../codes/java/438.找到字符串中所有字母异位词.java) |
[golang版本](../../../codes/golang/438.找到字符串中所有字母异位词.go) |
[python](../../../codes/python/438.找到字符串中所有字母异位词.py)

</br>

## 480. 滑动窗口中位数

> [题目描述](https://leetcode-cn.com/problems/sliding-window-median/)

**题目解法**: xx

[rust版本](../../../codes/rust/480.滑动窗口中位数.rs) |
[java版本](../../../codes/java/480.滑动窗口中位数.java) |
[golang版本](../../../codes/golang/480.滑动窗口中位数.go) |
[python](../../../codes/python/480.滑动窗口中位数.py)
