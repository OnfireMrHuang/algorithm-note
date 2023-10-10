/*
 * @lc app=leetcode.cn id=44 lang=rust
 *
 * [44] 通配符匹配
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let match_chars = s.chars().collect::<Vec<char>>();
        let pattern_chars = p.chars().collect::<Vec<char>>();
        let s_len = match_chars.len();
        let p_len = pattern_chars.len();
        let mut dp = vec![vec![false; p_len + 1]; s_len + 1];
        dp[0][0] = true;
        for i in 1..=p_len {
            if pattern_chars[i - 1] == '*' {
                dp[0][i] = true;
            } else {
                break;
            }
        }
        for i in 1..=s_len {
            for j in 1..=p_len {
                if pattern_chars[j - 1] == '*' {
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j - 1]
                        && (match_chars[i - 1] == pattern_chars[j - 1]
                            || pattern_chars[j - 1] == '?');
                }
            }
        }
        dp[s_len][p_len]
    }
}
// @lc code=end
