/*
 * @lc app=leetcode.cn id=516 lang=rust
 *
 * [516] 最长回文子序列
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        // base case
        let mut dp = vec![1; n];
        // 状态转移方程
        for i in (0..n - 1).rev() {
            let mut pre = 0;
            for j in i + 1..n {
                let temp = dp[j];
                if s.get(i..i + 1).unwrap() == s.get(j..j + 1).unwrap() {
                    dp[j] = pre + 2;
                } else {
                    dp[j] = std::cmp::max(dp[j], dp[j - 1])
                }
                pre = temp;
            }
        }
        dp[n - 1]
    }
}
// @lc code=end
