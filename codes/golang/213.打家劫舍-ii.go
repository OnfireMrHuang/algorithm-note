/*
 * @lc app=leetcode.cn id=213 lang=golang
 *
 * [213] 打家劫舍 II
 */

// @lc code=start
func rob(nums []int) int {
	size := len(nums)

	if size == 1 {
		return nums[0]
	}

	return max(robRange(nums,0,size-2),robRange(nums,1,size-1))
}

func robRange(nums []int,start,end int) int {

	dp_i,dp_i_1,dp_i_2 := 0,0,0

	for i := end; i >= start; i-- {
		dp_i = max(dp_i_1,nums[i] + dp_i_2)
		dp_i_2 = dp_i_1
		dp_i_1 = dp_i
	}
	return dp_i
}

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

