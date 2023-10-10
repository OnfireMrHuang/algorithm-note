/*
 * @lc app=leetcode.cn id=120 lang=rust
 *
 * [120] 三角形最小路径和
 */

// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let row_len = triangle.len();
        let col_len = triangle[row_len - 1].len();
        let mut dp = vec![0; row_len];
        dp[0] = triangle[0][0];
        for i in 1..row_len {
            dp[i] = dp[i - 1] + triangle[i][i];
            for j in (1..i).rev() {
                dp[j] = std::cmp::min(dp[j - 1], dp[j]) + triangle[i][j];
            }
            dp[0] += triangle[i][0];
        }
        let mut result = i32::MAX;
        for i in 0..row_len {
            if dp[i] < result {
                result = dp[i];
            }
        }
        result
    }
}
// @lc code=end
