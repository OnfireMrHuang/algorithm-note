/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut state = Vec::new();
        Self::backtrace(&mut state, &nums, 0, &mut res);
        res
    }

    fn backtrace(state: &mut Vec<i32>, nums: &Vec<i32>, start_idx: usize, res: &mut Vec<Vec<i32>>) {
        // 将子集加入到结果集中去
        res.push(state.clone());

        //开始选择
        for i in start_idx..nums.len() {
            state.push(nums[i]);
            Self::backtrace(state, nums, i + 1, res);
            state.pop();
        }
    }
}
// @lc code=end
