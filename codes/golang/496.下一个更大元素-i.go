

import "fmt"

func nextGreaterElement(nums1 []int, nums2 []int) []int {
	// 记录 nums2 中每个元素的下一个更大元素
	greater := nextGreater(nums2)
	// 转化成映射：元素 x -> x 的下一个最大元素
	greaterMap := make(map[int]int)
	for i, v := range nums2 {
		greaterMap[v] = greater[i]
	}
	// nums1 是 nums2 的子集，所以根据 greaterMap 可以得到结果
	res := make([]int, len(nums1))
	for i, v := range nums1 {
		res[i] = greaterMap[v]
	}
	return res
}

// 计算 nums 中每个元素的下一个更大元素
func nextGreater(nums []int) []int {
	n := len(nums)
	// 存放答案的数组
	res := make([]int, n)
	s := []int{}
	// 倒着往栈里放
	for i := n - 1; i >= 0; i-- {
		// 判定个子高矮
		for len(s) != 0 && s[len(s)-1] <= nums[i] {
			// 矮个起开，反正也被挡着了。。。
			s = s[:len(s)-1]
		}
		// nums[i] 身后的下一个更大元素
		if len(s) == 0 {
			res[i] = -1
		} else {
			res[i] = s[len(s)-1]
		}
		s = append(s, nums[i])
	}
	return res
}