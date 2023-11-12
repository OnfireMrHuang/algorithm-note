# 题解

## 496.下一个更大元素 I

> [题目描述](https://leetcode-cn.com/problems/next-greater-element-i/description/)

**题目解法**: 我们在[单调栈简要说明](./brief_introduction.md)中介绍了单调栈的一个示例，该题目明确说明<span style="color:red">nums1</span>是<span style="color:red">nums2</span>的一个子集，我们可以提前将<span style="color:red">nums2</span>中每个元素的下一个更大元素计算出来，然后在<span style="color:red">nums1</span>中查找即可。

[rust版本](../../../codes/rust/496.下一个更大元素-i.rs) |
[java版本](../../../codes/java/496.下一个更大元素-i.java) |
[golang版本](../../../codes/golang/496.下一个更大元素-i.go) |
[python版本](../../../codes/python/496.下一个更大元素-i.py)

</br>

## 503.下一个更大元素 II

> [题目描述](https://leetcode.cn/problems/next-greater-element-ii/description/)

**题目解法**:  这道题目很直觉的一个做法就是考虑将数组复制一份，然后拼接到原数组的后面，这样就可以将循环数组转换为线性数组，然后再使用单调栈来解决。

[rust版本](../../../codes/rust/503.下一个更大元素-ii.rs) |
[java版本](../../../codes/java/503.下一个更大元素-ii.java) |
[golang版本](../../../codes/golang/503.下一个更大元素-ii.go) |
[python版本](../../../codes/python/503.下一个更大元素-ii.py)

</br>

## 739.每日温度

> [题目描述](https://leetcode-cn.com/problems/daily-temperatures/)

**题目解法**:  这道题目和上面的两道题目类似，我们可以使用单调栈来解决，唯一不同的地方是，这道题目要求是记录与下一个更大元素的距离，所以我们在单调栈中存储的是元素的下标，而不是元素的值。

[rust版本](../../../codes/rust/739.每日温度.rs) |
[java版本](../../../codes/java/739.每日温度.java) |
[golang版本](../../../codes/golang/739.每日温度.go) |
[python版本](../../../codes/python/739.每日温度.py)

</br>

## 239.滑动窗口最大值

> [题目描述](https://leetcode-cn.com/problems/sliding-window-maximum/)

**题目解法**: 该题目就是[单调队列](./brief_introduction.md)的一个应用，我们先对nums数组的前K个元素初始化入单调队列，形成大小为k的window窗口(其中window中的最大值就是队首，因为入队时会把所有比该值小的元素全踢走，踢不走的就只剩下更大的或者自身)，然后开始向右滑动窗口，每次滑动窗口时，我们将下一个索引的值加入单调队列，同时对比队首的值和window窗口的左边界值，如果相等则说明队首的值已经不在window窗口中了，我们将其从队列中踢出，如果不想等则不处理，此时的队首元素就是滑动后的window窗口中的最大值，接着继续滑动直到滑到最后一个元素。

[rust版本](../../../codes/rust/239.滑动窗口最大值.rs) |
[java版本](../../../codes/java/239.滑动窗口最大值.java) |
[golang版本](../../../codes/golang/239.滑动窗口最大值.go) |
[python版本](../../../codes/python/239.滑动窗口最大值.py)

</br>
