/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len()];
        // base case
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);
        // 状态转移方程
        for i in 2..nums.len() {
            dp[i] = std::cmp::max(dp[i - 2] + nums[i], dp[i - 1]);
        }
        dp[nums.len() - 1]
    }
}
// @lc code=end
