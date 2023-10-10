/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 * [122] 买卖股票的最佳时机 II
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut min_price = std::i32::MAX;
        let mut max_profit = 0;
        let mut ans = 0;
        for price in prices {
            min_price = std::cmp::min(min_price, price);
            if price - min_price >= max_profit {
                max_profit = price - min_price;
            } else {
                ans += max_profit;
                min_price = price;
                max_profit = 0;
            }
        }
        ans + max_profit
    }
}
// @lc code=end
