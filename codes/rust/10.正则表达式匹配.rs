/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // 首先将字符串转换为字符数组
        let s_chars = s.chars().collect::<Vec<char>>();
        let p_chars = p.chars().collect::<Vec<char>>();
        let s_len = s_chars.len();
        let p_len = p_chars.len();
        // 定义一个动态规划的二维数组，其中f[i][j]表示s的前i个字符与p的前j个字符是否匹配
        let mut dp = vec![vec![false; p_len + 1]; s_len + 1];
        // 如果两个都为空串则一定匹配,如果p是空串则一定不匹配，如果s是空串而p是字符+*则可能匹配，因为字符+*可以匹配0次
        dp[0][0] = true;
        for i in 0..=s_len {
            for j in 1..=p_len {
                // 然后开始判断: 分为带*和不带*两种情况
                if p_chars[j - 1] == '*' {
                    // *需要和前面的一个字符结合起来看待，可以匹配0次、1次、N次
                    // 如果是匹配0次，那么就是将字符+*扣掉，所以是:
                    dp[i][j] = dp[i][j - 2];
                    // 匹配1次或N次
                    if Self::is_match_char(&s_chars, &p_chars, i, j - 1) {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                } else {
                    // 两边当前字符相等并且上一次也匹配
                    dp[i][j] = Self::is_match_char(&s_chars, &p_chars, i, j) && dp[i - 1][j - 1];
                }
            }
        }
        dp[s_len][p_len]
    }

    fn is_match_char(s_chars: &Vec<char>, p_chars: &Vec<char>, i: usize, j: usize) -> bool {
        // 边界条件： 如果字符串索引为0并且匹配字符串索引也为0那么返回false
        if i == 0 || j == 0 {
            return false;
        }
        if p_chars[j - 1] == '.' {
            return true;
        }
        s_chars[i - 1] == p_chars[j - 1]
    }
}
// @lc code=end
