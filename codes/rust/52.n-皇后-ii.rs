/*
 * @lc app=leetcode.cn id=52 lang=rust
 *
 * [52] N 皇后 II
 */

// @lc code=start
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        // 初始化n * n大小的棋盘，其中'Q'代表皇后，'#'代表空位。
        let mut state: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            let mut row: Vec<char> = Vec::new();
            for _ in 0..n {
                row.push('#');
            }
            state.push(row);
        }
        // 记录列是否有皇后
        let mut cols = vec![false; n as usize];
        // 记录主对角线是否有皇后
        let mut diags1 = vec![false; 2 * n as usize - 1];
        // 记录副对角线是否有皇后
        let mut diags2 = vec![false; 2 * n as usize - 1];
        let mut cnt = 0;
        Self::backtrace(
            0,
            n as usize,
            &mut state,
            &mut cols,
            &mut diags1,
            &mut diags2,
            &mut cnt,
        );
        cnt
    }

    fn backtrace(
        row: usize,
        n: usize,
        state: &mut Vec<Vec<char>>,
        cols: &mut Vec<bool>,
        diags1: &mut Vec<bool>,
        diags2: &mut Vec<bool>,
        ans: &mut i32,
    ) {
        // 当放置完成所有行时,返回n皇后成立
        if row == n {
            // 递增结果
            *ans = *ans + 1;
            return;
        }
        // 遍历所有列
        for col in 0..n {
            // 计算该格子对应的主对角线和副对角线
            let diag1 = row + n - 1 - col;
            let diag2 = row + col;
            // 剪枝: 不允许该格子所在列、主对角线、副对角线存在皇后
            if !cols[col] && !diags1[diag1] && !diags2[diag2] {
                // 检查都合法，将皇后放置在格子中
                state.get_mut(row).unwrap()[col] = 'Q';
                // 将状态置位
                cols[col] = true;
                diags1[row + n - 1 - col] = true;
                diags2[row + col] = true;
                // 接着开始回溯下一行
                Self::backtrace(row + 1, n, state, cols, diags1, diags2, ans);
                // 回退状态
                cols[col] = false;
                diags1[row + n - 1 - col] = false;
                diags2[row + col] = false;
                state.get_mut(row).unwrap()[col] = '#';
            }
        }
    }
}
// @lc code=end
