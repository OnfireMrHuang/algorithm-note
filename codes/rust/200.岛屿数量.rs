/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */

// @lc code=start
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut num_islands = 0;
        let row_len = grid.len();
        let col_len = grid[0].len();
        for row in 0..row_len {
            for col in 0..col_len {
                if grid[row][col] == '1' {
                    num_islands += 1;
                    Self::dfs(&mut grid, row as i32, col as i32);
                }
            }
        }
        num_islands
    }

    fn dfs(grid: &mut Vec<Vec<char>>, row: i32, col: i32) {
        let row_len = grid.len();
        let col_len = grid[0].len();

        if row < 0
            || col < 0
            || row as usize >= row_len
            || col as usize >= col_len
            || grid[row as usize][col as usize] == '0'
        {
            return;
        }

        grid[row as usize][col as usize] = '0';

        Self::dfs(grid, row - 1, col);
        Self::dfs(grid, row + 1, col);
        Self::dfs(grid, row, col - 1);
        Self::dfs(grid, row, col + 1);
    }
}
// @lc code=end
