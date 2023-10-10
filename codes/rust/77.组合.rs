/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 */

// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut state = Vec::new();
        Self::backtrace(state, n, 1, k as usize, &mut result);
        result
    }

    fn backtrace(mut state: Vec<i32>, n: i32, start_num: i32, k: usize, res: &mut Vec<Vec<i32>>) {
        // 递归的底部回退
        if state.len() == k {
            res.push(state);
            return;
        }

        //开始选择
        for num in start_num..=n {
            state.push(num);
            Self::backtrace(state.clone(), n, num + 1, k, res);
            state.pop();
        }
    }
}
// @lc code=end
