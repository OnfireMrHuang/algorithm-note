/*
 * @lc app=leetcode.cn id=714 lang=rust
 *
 * [714] 买卖股票的最佳时机含手续费
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let size = prices.len();
        let mut dp = vec![vec![0; 2]; size];
        // 持有第一只股票，金额减去prices[0],不持有则保持金额为0不变
        dp[0][0] = -1 * prices[0];
        for i in 1..size {
            // 持有股票推算
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][1] - prices[i]);
            dp[i][1] = std::cmp::max(dp[i - 1][1], dp[i - 1][0] + prices[i] - fee);
        }
        // 最后肯定是不持有股票
        dp[size - 1][1]
    }
}
// @lc code=end
