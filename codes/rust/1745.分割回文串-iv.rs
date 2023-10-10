/*
 * @lc app=leetcode.cn id=1745 lang=rust
 *
 * [1745] 分割回文串 IV
 */

// @lc code=start
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let size = s.len();
        // 预计算构造回文串判断表
        let mut palindrome_table = vec![vec![false; size]; size];
        Self::construct_palindrome_table(&s, &mut palindrome_table);
        let mut dp = vec![vec![false; 3]; size];
        dp[0][0] = true;
        for i in 1..size {
            for j in 0..=std::cmp::min(i, 2) {
                if j == 0 {
                    dp[i][j] = palindrome_table[0][i];
                } else {
                    for m in j - 1..i {
                        dp[i][j] = dp[i][j] || (dp[m][j - 1] && palindrome_table[m + 1][i]);
                    }
                }
            }
        }
        dp[size - 1][2]
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
