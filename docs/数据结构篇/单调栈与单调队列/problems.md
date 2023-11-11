# 题解

## 496.下一个更大元素 I

**题目描述**: 

<span style="color:red">nums1</span>中数字<span style="color:red">x</span>的**下一个更大元素**是指 <span style="color:red">x</span>在 nums2 中对应位置**右侧**的**第一个**比<span style="color:red">x</span>大的元素。

给你两个 没有重复元素 的数组 nums1 和 nums2 ，下标从 0 开始计数，其中nums1 是 nums2 的子集。

对于每个 0 <= i < nums1.length ，找出满足 nums1[i] == nums2[j] 的下标 j ，并且在 nums2 确定 nums2[j] 的 下一个更大元素 。如果不存在下一个更大元素，那么本次查询的答案是 -1 。

返回一个长度为 nums1.length 的数组 ans 作为答案，满足 ans[i] 是如上所述的 下一个更大元素 。

**示例**：

```text
输入：nums1 = [4,1,2], nums2 = [1,3,4,2]
输出：[-1,3,-1]
解释：nums1 中每个值的下一个更大元素如下所述：
- 4 ，用加粗斜体标识，nums2 = [1,3,4,2]。不存在下一个更大元素，所以答案是 -1 。
- 1 ，用加粗斜体标识，nums2 = [1,3,4,2]。下一个更大元素是 3 。
- 2 ，用加粗斜体标识，nums2 = [1,3,4,2]。不存在下一个更大元素，所以答案是 -1 。
```

**提示**：

- 1 <= nums1.length <= nums2.length <= 1000
- 0 <= nums1[i], nums2[i] <= 104
- nums1和nums2中所有整数 互不相同
- nums1 中的所有整数同样出现在 nums2 中

**题目解法**: 我们在[单调栈简要说明](./brief_introduction.md)中介绍了单调栈的一个示例，该题目明确说明<span style="color:red">nums1</span>是<span style="color:red">nums2</span>的一个子集，我们可以提前将<span style="color:red">nums2</span>中每个元素的下一个更大元素计算出来，然后在<span style="color:red">nums1</span>中查找即可。

[rust版本](../../../codes/rust/496.下一个更大元素-i.rs) |
[java版本](../../../codes/java/496.下一个更大元素-i.java) |
[golang版本](../../../codes/golang/496.下一个更大元素-i.go) |
[python版本](../../../codes/python/496.下一个更大元素-i.py)

<br>

## 503.下一个更大元素 II

<br>


## 239.滑动窗口最大值

<br>
