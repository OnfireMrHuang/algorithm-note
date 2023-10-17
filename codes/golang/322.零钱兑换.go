/*
 * @lc app=leetcode.cn id=322 lang=golang
 *
 * [322] 零钱兑换
 */

import "math"

// 动态规划题目解法
// 1、base case -- 当amount为0时，所需的肯定为0，当amount小于0时，所需的硬币肯定没有
// 2、状态 -- 硬币面额是固定的，为了向base case靠拢，状态变量肯定是amount
// 3、状态转换 -- 当我们挑一枚硬币的时候，amount就减去一张面额，同时减去一颗硬币
// 4、dp函数/数组定义，参数肯定是我们的变量，返回值则肯定是我们的硬币数量

// 自定向下
// @lc code=start
// func coinChange(coins []int, amount int) int {
// 	memo := make(map[int]int)
// 	return dp(coins, amount, &memo)
// }

// func dp(coins []int, amount int, memo *map[int]int) int {
// 	if amount == 0 {
// 		return 0
// 	}
// 	if amount < 0 {
// 		return -1
// 	}
// 	result := math.MaxInt32
// 	for i := 0; i < len(coins); i++ {
// 		var existed bool
// 		var subProblem int
// 		if subProblem, existed = (*memo)[amount-coins[i]]; !existed {
// 			subProblem = coinChange(coins, amount-coins[i])
// 		}
// 		if subProblem == -1 {
// 			continue // 无解，跳过
// 		}
// 		result = min(result, 1+subProblem)
// 	}
// 	if result == math.MaxInt32 {
// 		return -1 // 都没找到，返回-1
// 	}
// 	return result
// }

// 自下向上
func coinChange(coins []int, amount int) int {

	dp := make([]int, amount+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = amount + 1
	}
	dp[0] = 0 // base case

	var i int
	var j int
	for i = 0; i < len(dp); i++ {
		for j = 0; j < len(coins); j++ {
			if i-coins[j] < 0 {
				continue
			}
			dp[i] = min(dp[i], 1+dp[i-coins[j]])
		}
	}
	if dp[amount] == amount+1 {
		return -1
	}
	return dp[amount]
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}

// @lc code=end

