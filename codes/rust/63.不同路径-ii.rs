/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        // 初始化第一行: 如果中间有障碍物，那么后面的都到不了
        let mut dp_above = vec![0; n as usize];
        for i in 0..n as usize {
            if obstacle_grid[0][i] == 1 {
                break;
            }
            dp_above[i] = 1;
        }
        let mut dp_left = 0;
        let mut dp = dp_above[n as usize - 1];
        for i in 1..m as usize {
            dp_left = 0;
            for j in 0..n as usize {
                // 如果当前是障碍物，那么更新左边的dp为0以及上边的dp为0
                if obstacle_grid[i][j] == 1 {
                    dp = 0;
                    dp_above[j] = 0;
                    dp_left = 0;
                    continue;
                }
                dp = dp_above[j] + dp_left;
                dp_left = dp;
                dp_above[j] = dp;
            }
        }
        dp
    }
}
// @lc code=end
