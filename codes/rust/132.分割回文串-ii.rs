/*
 * @lc app=leetcode.cn id=132 lang=rust
 *
 * [132] 分割回文串 II
 */

// @lc code=start
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }
        let mut table: Vec<Vec<bool>> = vec![vec![false; s.len()]; s.len()];
        Self::construct_palindrome_table(&s, &mut table);
        let mut dp = vec![i32::MAX; s.len()];
        // base case: 第一个字符不用分割
        dp[0] = 0;
        // 从第二个字符开始递推
        for i in 1..s.len() {
            // 如果第一个字符到i字符是回文串则没有分割次数
            if table[0][i] {
                dp[i] = 0;
                continue;
            }
            // 否则如果中间字符到i字符是回文串则+1
            for cur in 1..=i {
                if !table[cur][i] {
                    continue;
                }
                dp[i] = std::cmp::min(dp[i], dp[cur - 1] + 1);
            }
        }
        dp[s.len() - 1]
    }

    fn construct_palindrome_table(s: &str, table: &mut Vec<Vec<bool>>) {
        let size = s.len();
        for i in (0..size).rev() {
            for j in i..size {
                //单字符时是回文串
                if j == i {
                    table[i][j] = true;
                    continue;
                }
                // 相邻字符并且相等时是回文串
                if i + 1 == j && &s[i..i + 1] == &s[j..j + 1] {
                    table[i][j] = true;
                    continue;
                }
                // 否则根据内层是否是回文串判定
                table[i][j] = &s[i..i + 1] == &s[j..j + 1] && table[i + 1][j - 1];
            }
        }
    }
}
// @lc code=end
