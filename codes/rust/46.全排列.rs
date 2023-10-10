/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut state = Vec::new();
        let mut selected = vec![false; nums.len()];
        let choices = nums;
        Self::backtrack(state, &choices, &mut selected, &mut result);
        result
    }

    fn backtrack(
        mut state: Vec<i32>,
        choices: &Vec<i32>,
        selected: &mut Vec<bool>,
        result: &mut Vec<Vec<i32>>,
    ) {
        // 如果最终状态的长度等于可选择的状态长度则添加结果并返回
        if state.len() == choices.len() {
            result.push(state);
            return;
        }
        // 开始选择数值
        for i in 0..choices.len() {
            // 查看是否被选择过
            if selected[i] {
                continue;
            }
            selected[i] = true;
            state.push(choices[i]);
            Self::backtrack(state.clone(), choices, selected, result);
            selected[i] = false;
            state.pop();
        }
    }
}
// @lc code=end
