/*
 * @lc app=leetcode.cn id=1312 lang=golang
 *
 * [1312] 让字符串成为回文串的最少插入次数
 */

// @lc code=start
func minInsertions(s string) int {
	size := len(s)
	// 构造dp数组
	dp := make([]int, size)
	var temp int
	// 倒序遍历dp
	for i:= size-2; i >= 0; i-- {
		pre := 0
		for j := i+1; j < size; j++ {
			temp = dp[j]
			if s[i] == s[j] {
				dp[j] = pre
			} else {
				dp[j] = min(dp[j],dp[j-1])+1
			}
			pre = temp
		}
	}
	return dp[size-1]
}

func min(a,b int) int {
	if a > b {
		return b
	}
	return a
}
// @lc code=end

