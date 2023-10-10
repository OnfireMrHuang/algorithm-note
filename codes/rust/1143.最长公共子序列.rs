/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 */

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if text1.get(i - 1..i) == text2.get(j - 1..j) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[m][n]
    }
}
// @lc code=end
