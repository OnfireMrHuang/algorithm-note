/*
 * @lc app=leetcode.cn id=516 lang=golang
 *
 * [516] 最长回文子序列
 */

// @lc code=start
// 状态压缩，每次计算只需要上一次的dp数值
func longestPalindromeSubseq(s string) int {
	size := len(s)
	// 初始化dp数组
	dp := make([]int, size)
	for i :=0; i<size; i++ {
		// dp[i] = make([]int, size)
		dp[i] = 1
	}
	var pre int
	// 开始返向遍历
	for i := size-2; i>=0; i-- {
		pre = 0
		for j := i+1; j<size; j++ {
			temp := dp[j]
			if s[i] == s[j] {
				//dp[i][j] = dp[i+1][j-1] + 2
				dp[j] = pre + 2
			} else {
				//dp[i][j] = max(dp[i+1][j],dp[i][j-1])
				dp[j] = max(dp[j],dp[j-1])
			}
			pre = temp
		}
	}
	return dp[size-1]
}

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

