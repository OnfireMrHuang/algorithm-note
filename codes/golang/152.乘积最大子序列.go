/*
 * @lc app=leetcode.cn id=152 lang=golang
 *
 * [152] 乘积最大子序列
 */

// @lc code=start
func maxProduct(nums []int) int {

	minimum,maximum,res := nums[0],nums[0],nums[0]

	for i:=1;i<len(nums);i++ {
		if nums[i] <0 {
			maximum, minimum = minimum, maximum
		}
		maximum = max(maximum * nums[i],nums[i])
		minimum = min(minimum * nums[i],nums[i])
		res = max(res,maximum)
	}
	return res
}

func max(x,y int) int {
	if x >y {
		return x
	}
	return y
}

func min(x,y int) int {
	if x <y {
		return x
	}
	return y
}

// @lc code=end

