/*
 * @lc app=leetcode.cn id=887 lang=golang
 *
 * [887] 鸡蛋掉落
 */

// @lc code=start

// 修改状态转移，dp数组里面存放给你k个鸡蛋，测试m次，最坏的情况下测试n层楼

// import "math"

func superEggDrop(k int, n int) int {
	// 初始化dp数组
	dp := make([][]int, k+1)
	for i:=0; i<=k;i++ {
		dp[i] = make([]int, n+1)
	}
	// return dp(k,n,table)
	var m int
	for dp[k][m] < n {
		m++
		for i:=1; i<=k; i++ {
			dp[i][m] = dp[i][m-1] + dp[i-1][m-1] + 1
		}
	}
	return m
}

func min(a,b int) int {
	if a < b {
		return a
	}	
	return b
}

func max(a,b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

