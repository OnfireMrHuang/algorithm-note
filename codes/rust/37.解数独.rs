/*
 * @lc app=leetcode.cn id=37 lang=rust
 *
 * [37] 解数独
 */

// @lc code=start
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtrace(0, 0, board);
    }

    fn backtrace(row: usize, col: usize, board: &mut Vec<Vec<char>>) -> bool {
        // 如果超过最后一行，则返回true
        if row >= board.len() {
            return true;
        }
        let mut next_row = row;
        let mut next_col = col + 1;
        if next_col >= board.len() {
            next_row = row + 1;
            next_col = 0;
        }
        // 如果格子里面已经有数字了，则前往下一个格子
        if board[row][col] != '.' {
            return Self::backtrace(next_row, next_col, board);
        }
        // 如果格子里面没有数字，则尝试填入数字
        for i in '1'..='9' {
            if Self::is_valid(row, col, i, board) {
                // 将单元格填成数字
                board[row][col] = i;
                if Self::backtrace(next_row, next_col, board) {
                    return true;
                }
                // 如果没有成功则改回.
                board[row][col] = '.';
            }
        }
        // 没有匹配的数字，返回false
        false
    }

    fn is_valid(row: usize, col: usize, num: char, board: &mut Vec<Vec<char>>) -> bool {
        // 检查行
        for i in 0..board.len() {
            if board[i][col] == num {
                return false;
            }
        }
        // 检查列
        for j in 0..board.len() {
            if board[row][j] == num {
                return false;
            }
        }
        // 检查3x3
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for i in start_row..(start_row + 3) {
            for j in start_col..(start_col + 3) {
                if board[i][j] == num {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end
