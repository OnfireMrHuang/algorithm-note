/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
impl Solution {
    pub fn majority_element(param: Vec<i32>) -> i32 {
        let mut nums = param;
        nums.sort();
        return nums[nums.len() / 2];
    }
}
// @lc code=end
