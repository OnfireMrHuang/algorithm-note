/*
 * @lc app=leetcode.cn id=312 lang=golang
 *
 * [312] 戳气球
 */

// @lc code=start
func maxCoins(nums []int) int {
	size := len(nums)
	points := make([]int, size+2)
	points[0] = 1
	points[size+1]=1
	for i := 1; i < size+1; i++ {
		points[i] = nums[i-1]
	}

	// 定义dp数组
	dp := make([][]int,size+2)
	for i := 0; i < (size+2); i++ {
		dp[i] = make([]int,size+2)
	}	

	// 循环定义
	n := len(points)-1
	for i := size; i >=0 ; i++ {
		for j := i+1; j < size+2; j++ {
			for k := i+1; k < j; k++ {
				dp[i][j] = max(dp[i][j],
				dp[i][k] + dp[k][j] + points[i] * points[j] * points[k])
			}
		}
	}
	return dp[0][n+1]
}

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

