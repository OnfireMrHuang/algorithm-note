/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 */

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut dp = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1)
                }
            }
        }
        for val in dp {
            result = std::cmp::max(result, val);
        }
        result
    }
}
// @lc code=end
