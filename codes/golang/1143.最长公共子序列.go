/*
 * @lc app=leetcode.cn id=1143 lang=golang
 *
 * [1143] 最长公共子序列
 */

// @lc code=start
func longestCommonSubsequence(text1 string, text2 string) int {
	// return dp(text1,text2,len(text1)-1,len(text2)-1)
	m := len(text1)
	n := len(text2)
	dp := make([][]int,m+1)
	for i:=0; i <= m; i++ {
		dp[i] = make([]int, n+1)
	}

	for i:=1; i <=m; i++ {
		for j := 1; j <=n; j++ {
			if text1[i-1] == text2[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
			} else {
				dp[i][j] = max(dp[i-1][j],dp[i][j-1])
			}
		}
	}
	return dp[m][n]
}

// func dp(text1,text2 string,i,j int) int {
// 	if i == -1 || j == -1 {
// 		return 0
// 	}
// 	if text1[i] == text2[j] {
// 		return dp(text1,text2,i-1,j-1) + 1
// 	} 
// 	return max(dp(text1,text2,i,j-1),dp(text1,text2,i-1,j)) 
// }

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

