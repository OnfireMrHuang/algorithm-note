/*
 * @lc app=leetcode.cn id=120 lang=golang
 *
 * [120] 三角形最小路径和
 */

// @lc code=start
func minimumTotal(triangle [][]int) int {

	dp := make([]int,len(triangle)+1)

	for i:=len(triangle)-1;i>=0;i-- {
		for j:=0;j < len(triangle[i]);j++ {
			dp[j] = min(dp[j],dp[j+1])+triangle[i][j]			
		}
	}
	return dp[0]
}

func min(x,y int) int {
	if x > y {
		return y
	}
	return x
}
// @lc code=end

