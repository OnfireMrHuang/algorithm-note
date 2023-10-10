/*
 * @lc app=leetcode.cn id=36 lang=rust
 *
 * [36] 有效的数独
 */

// @lc code=start
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let row_len = board.len();
        if row_len < 1 {
            return false;
        }
        let col_len = board[0].len();
        let mut row_flag: Vec<Vec<bool>> = vec![vec![false; 10]; row_len];
        let mut col_flag: Vec<Vec<bool>> = vec![vec![false; 10]; row_len];
        let mut box_flag: Vec<Vec<bool>> = vec![vec![false; 10]; row_len];
        for i in 0..row_len {
            for j in 0..col_len {
                if board[i][j] == '.' {
                    continue;
                }
                let num = board[i][j] as usize - '0' as usize;
                if row_flag[i][num] {
                    return false;
                }
                if col_flag[j][num] {
                    return false;
                }
                if box_flag[(i / 3) * 3 + j / 3][num] {
                    return false;
                }
                row_flag[i][num] = true;
                col_flag[j][num] = true;
                box_flag[(i / 3) * 3 + j / 3][num] = true;
            }
        }
        true
    }
}
// @lc code=end
