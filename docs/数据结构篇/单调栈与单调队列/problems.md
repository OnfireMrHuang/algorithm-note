# 题解

## 496.下一个更大元素 I

> [题目描述](https://leetcode-cn.com/problems/next-greater-element-i/description/)

**题目解法**: 我们在[单调栈简要说明](./brief_introduction.md)中介绍了单调栈的一个示例，该题目明确说明<span style="color:red">nums1</span>是<span style="color:red">nums2</span>的一个子集，我们可以提前将<span style="color:red">nums2</span>中每个元素的下一个更大元素计算出来，然后在<span style="color:red">nums1</span>中查找即可。

[rust版本](../../../codes/rust/496.下一个更大元素-i.rs) |
[java版本](../../../codes/java/496.下一个更大元素-i.java) |
[golang版本](../../../codes/golang/496.下一个更大元素-i.go) |
[python版本](../../../codes/python/496.下一个更大元素-i.py)

<br>

## 503.下一个更大元素 II

> [题目描述](https://leetcode.cn/problems/next-greater-element-ii/description/)

**题目解法**:  这道题目很直觉的一个做法就是考虑将数组复制一份，然后拼接到原数组的后面，这样就可以将循环数组转换为线性数组，然后再使用单调栈来解决。

[rust版本](../../../codes/rust/503.下一个更大元素-ii.rs) |
[java版本](../../../codes/java/503.下一个更大元素-ii.java) |
[golang版本](../../../codes/golang/503.下一个更大元素-ii.go) |
[python版本](../../../codes/python/503.下一个更大元素-ii.py)

<br>


## 239.滑动窗口最大值

> [题目描述](https://leetcode-cn.com/problems/sliding-window-maximum/)

**题目解法**:

[rust版本]

<br>
