/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount as usize {
            for j in 0..coins.len() {
                if coins[j] <= i as i32 {
                    dp[i] = dp[i].min(dp[i - coins[j] as usize] + 1);
                }
            }
        }
        let mut result = 0;
        if dp[amount as usize] > amount {
            result = -1;
        } else {
            result = dp[amount as usize];
        }
        result
    }
}
// @lc code=end
