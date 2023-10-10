/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 */

// @lc code=start
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 1 {
            return 0;
        }
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut dp = vec![vec![0; col_len]; row_len];
        for i in 0..row_len {
            for j in 0..col_len {
                let mut last_min_sum = 0;
                if i > 0 && j > 0 {
                    last_min_sum = dp[i - 1][j].min(dp[i][j - 1]);
                } else if i <= 0 && j > 0 {
                    last_min_sum = dp[i][j - 1];
                } else if i > 0 && j <= 0 {
                    last_min_sum = dp[i - 1][j];
                }
                dp[i][j] = last_min_sum + grid[i][j];
            }
        }

        dp[row_len - 1][col_len - 1]
    }
}
// @lc code=end
