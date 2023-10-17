/*
 * @lc app=leetcode.cn id=88 lang=golang
 *
 * [88] 合并两个有序数组
 *
 * https://leetcode-cn.com/problems/merge-sorted-array/description/
 *
 * algorithms
 * Easy (42.76%)
 * Total Accepted:    29.1K
 * Total Submissions: 68.1K
 * Testcase Example:  '[1,2,3,0,0,0]\n3\n[2,5,6]\n3'
 *
 * 给定两个有序整数数组 nums1 和 nums2，将 nums2 合并到 nums1 中，使得 num1 成为一个有序数组。
 *
 * 说明:
 *
 *
 * 初始化 nums1 和 nums2 的元素数量分别为 m 和 n。
 * 你可以假设 nums1 有足够的空间（空间大小大于或等于 m + n）来保存 nums2 中的元素。
 *
 *
 * 示例:
 *
 * 输入:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 *
 * 输出: [1,2,2,3,5,6]
 *
 */

//使用归并排序,归这部分已经给出来了，现在是并的阶段
func merge(nums1 []int, m int, nums2 []int, n int) {
	//首先将nums1拷贝出来
	temp := make([]int, m)
	copy(temp, nums1)

	i, j, k := 0, 0, 0
	for i = 0; i < m+n; i++ {

		if j >= m {
			nums1[i] = nums2[k]
			k++
			continue
		}

		if k >= n {
			nums1[i] = temp[j]
			j++
			continue
		}

		if temp[j] > nums2[k] {
			nums1[i] = nums2[k]
			k++
		} else {
			nums1[i] = temp[j]
			j++
		}
	}
}
