/*
 * @lc app=leetcode.cn id=300 lang=golang
 *
 * [300] 最长上升子序列
 */

// @lc code=start
func lengthOfLIS(nums []int) int {
	dp := make([]int,len(nums))
	// dp数组全部初始化为1，因为每个元素至少包括它自身这么一个序列
	for i:=0; i < len(nums); i++ {
		dp[i] = 1
	}
	// 遍历数组
	for i := 0; i < len(nums); i++ {
		for j :=0; j < i; j++ {
			if nums[i] > nums[j] {
				dp[i] = max(dp[i],dp[j]+1)
			}
		}
	}
	var res  = 0
	for i:=0; i < len(dp); i++ {
		res = max(res,dp[i])
	}
	return res
}

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

