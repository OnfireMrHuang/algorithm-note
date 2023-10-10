/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 * [309] 买卖股票的最佳时机含冷冻期
 */

// @lc code=start
impl Solution {
    /*
    该题因为冷冻期的存在，不能再使用贪心算法(即局部的最优并不能达成全局的最优)，于是转换为使用动态规划解法:
    假设dp[i][j] = val; 其中i表示在[0..i]天，j表示[0,1,2,3],其中0表示不操作，1表示买入，2表示卖出，3表示继续持有,val表示最大收益
    base case:
    dp[0][0] = 0, dp[0][1] = -prices[0], dp[0][2]=-2000, dp[0][3]=-2000; // 第一天不能卖或继续持有，所以直接设置
    动态转移方程:
    dp[i][0] = max(dp[i-1][0], dp[i-1][2])
    dp[i][1] = dp[i-1][0] - prices[i]
    dp[i][2] = max(dp[i-1][1] + prices[i], dp[i-1][3] + prices[i])
    dp[i][3] = max(dp[i-1][1], dp[i-1][3])
    */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 定义状态值
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 4]; prices.len()];
        // base case
        dp[0][1] = -1 * prices[0];
        dp[0][2] = -2000;
        dp[0][3] = -2000;
        for i in 1..prices.len() {
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][2]);
            dp[i][1] = dp[i - 1][0] - prices[i];
            dp[i][2] = std::cmp::max(dp[i - 1][1] + prices[i], dp[i - 1][3] + prices[i]);
            dp[i][3] = std::cmp::max(dp[i - 1][1], dp[i - 1][3]);
        }
        let mut max_profit = 0;
        for j in 0..4 {
            max_profit = std::cmp::max(max_profit, dp[prices.len() - 1][j]);
        }
        max_profit
    }
}
// @lc code=end
