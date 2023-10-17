/*
 * @lc app=leetcode.cn id=300 lang=golang
 *
 * [300] 最长上升子序列
 */

import "sort"

// @lc code=start
func lengthOfLIS(nums []int) int {
	dp := []int{}

	for _,num := range nums {
		i := sort.SearchInts(dp,num)
		if i == len(dp) {
			dp = append(dp,num)
		} else {
			dp[i] = num
		}
	}
	return len(dp)
}
// @lc code=end

