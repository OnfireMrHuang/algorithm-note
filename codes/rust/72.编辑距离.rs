/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        // 初始化base case
        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }
        // 开始状态转
        for i in 1..=m {
            for j in 1..=n {
                if word1.get(i - 1..i) == word2.get(j - 1..j) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    let del_op = dp[i - 1][j] + 1; // 删除操作
                    let ins_op = dp[i][j - 1] + 1; // 插入操作
                    let rep_op = dp[i - 1][j - 1] + 1; // 替换操作
                    dp[i][j] = std::cmp::min(del_op, ins_op).min(rep_op);
                }
            }
        }
        dp[m][n]
    }
}
// @lc code=end
