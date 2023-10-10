/*
 * @lc app=leetcode.cn id=312 lang=rust
 *
 * [312] 戳气球
 */

// @lc code=start
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut points = vec![0; n + 2];
        points[0] = 1;
        points[n + 1] = 1;
        for i in 0..n {
            points[i + 1] = nums[i];
        }
        let mut dp = vec![vec![0; n + 2]; n + 2];
        // 定义转移方程
        for i in (0..=n).rev() {
            for j in i + 1..n + 2 {
                for k in i + 1..j {
                    dp[i][j] = std::cmp::max(
                        dp[i][j],
                        dp[i][k] + dp[k][j] + points[k] * points[i] * points[j],
                    )
                }
            }
        }
        dp[0][n + 1]
    }
}
// @lc code=end
