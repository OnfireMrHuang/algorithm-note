/*
 * @lc app=leetcode.cn id=123 lang=rust
 *
 * [123] 买卖股票的最佳时机 III
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let size = prices.len();
        /*
        定义5中状态:
        0: 没有操作， 1: 第一次买入, 2: 第一次卖出, 3: 第二次买入, 4: 第二次卖出
         */
        let mut dp = vec![vec![0; 5]; size];
        dp[0][1] = -prices[0];
        dp[0][3] = -prices[0];
        for i in 1..size {
            dp[i][1] = std::cmp::max(dp[i - 1][1], dp[i - 1][0] - prices[i]);
            dp[i][2] = std::cmp::max(dp[i - 1][2], dp[i - 1][1] + prices[i]);
            dp[i][3] = std::cmp::max(dp[i - 1][3], dp[i - 1][2] - prices[i]);
            dp[i][4] = std::cmp::max(dp[i - 1][4], dp[i - 1][3] + prices[i]);
        }
        dp[size - 1][4]
    }
}
// @lc code=end
