/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        // 定义状态
        let mut dp1 = 0;
        let mut dp1_0 = nums[0];
        let mut dp1_1 = nums[0];
        let mut dp2 = 0;
        let mut dp2_0 = 0;
        let mut dp2_1 = nums[1];
        for i in 2..n - 1 {
            dp1 = std::cmp::max(dp1_0 + nums[i], dp1_1);
            dp1_0 = dp1_1;
            dp1_1 = dp1;
        }
        for i in 2..n {
            dp2 = std::cmp::max(dp2_0 + nums[i], dp2_1);
            dp2_0 = dp2_1;
            dp2_1 = dp2;
        }
        std::cmp::max(dp1_1, dp2_1)
    }
}
// @lc code=end
