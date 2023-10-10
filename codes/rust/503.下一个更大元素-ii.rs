/*
 * @lc app=leetcode.cn id=503 lang=rust
 *
 * [503] 下一个更大元素 II
 */

// @lc code=start
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; nums.len() * 2];
        let mut stack = Vec::new();
        let double = nums.repeat(2);
        for (idx, v) in double.iter().enumerate() {
            while !stack.is_empty() && double[*stack.last().unwrap()] < *v {
                let last_idx = stack.pop().unwrap();
                result[last_idx] = *v;
            }
            stack.push(idx);
        }
        result.into_iter().take(nums.len()).collect()
    }
}
// @lc code=end
