/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = std::i32::MAX;
        let mut max_profit = 0;
        for price in prices {
            min_price = std::cmp::min(min_price, price);
            if price - min_price > max_profit {
                max_profit = price - min_price;
            }
        }
        max_profit
    }
}
// @lc code=end
