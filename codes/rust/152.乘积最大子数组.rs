/*
 * @lc app=leetcode.cn id=152 lang=rust
 *
 * [152] 乘积最大子数组
 */

// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut result = nums[0];
        let mut max_dp = nums[0];
        let mut min_dp = nums[0];
        for i in 1..nums.len() {
            let (mx, mn) = (max_dp, min_dp);
            max_dp = std::cmp::max(std::cmp::max(mx * nums[i], nums[i]), mn * nums[i]);
            min_dp = std::cmp::min(std::cmp::min(mx * nums[i], nums[i]), mn * nums[i]);
            result = std::cmp::max(result, max_dp);
        }
        result
    }
}
// @lc code=end
