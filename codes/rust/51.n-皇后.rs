/*
 * @lc app=leetcode.cn id=51 lang=rust
 *
 * [51] N 皇后
 */

// @lc code=start
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        for i in 0..n {
            let mut board_row = vec!['.'; n as usize];
            let mut board: Vec<String> = vec![board_row.iter().collect::<String>(); n as usize];
            board_row[i as usize] = 'Q';
            board[0] = board_row.iter().collect::<String>();
            Self::backtrace(&mut board, 1, &mut result);
        }
        result
    }

    fn backtrace(board: &mut Vec<String>, row: usize, res: &mut Vec<Vec<String>>) {
        let n = board.len();
        if row == n {
            res.push(board.clone());
            return;
        }
        for col in 0..n {
            if !Self::is_valid(board, row, col) {
                continue;
            }
            // 选择放置皇后
            board[row].replace_range(col..col + 1, "Q");
            Self::backtrace(board, row + 1, res);
            // 撤销选择
            board[row].replace_range(col..col + 1, ".");
        }
    }

    fn is_valid(board: &Vec<String>, row: usize, col: usize) -> bool {
        // 判断正上方
        for i in 0..row {
            if let Some(queue) = board[i].get(col..col + 1) {
                if queue == "Q" {
                    return false;
                }
            }
        }

        // 判断左上方
        for (i, j) in (0..row).rev().zip((0..col).rev()) {
            if let Some(queue) = board[i].get(j..j + 1) {
                if queue == "Q" {
                    return false;
                }
            }
        }

        // 判断右上方
        for (i, j) in (0..row).rev().zip(col + 1..board.len()) {
            if let Some(queue) = board[i].get(j..j + 1) {
                if queue == "Q" {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end
