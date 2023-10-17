/*
 * @lc app=leetcode.cn id=518 lang=golang
 *
 * [518] 零钱兑换 II
 */

// @lc code=start
// 使用动态规划中的背包问题来解决。
// 凑数金额表示背包容量
// 硬币面值表示装包的物品
// 所以dp数组定义为dp[i][j] = count,i表示使用前i个面值，凑成金额j一共有有多少组合
func change(amount int, coins []int) int {
	size := len(coins)

	dp := make([]int,amount+1)
	dp[0] = 1

	for i := 0; i<size ; i++ {
		for j := 1; j <= amount; j++ {
			if j - coins[i] >= 0 {
				dp[j] = dp[j] + dp[j-coins[i]]
			}
		}
	}
	return dp[amount]
}
// @lc code=end

