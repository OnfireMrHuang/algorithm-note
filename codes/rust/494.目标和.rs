/*
 * @lc app=leetcode.cn id=494 lang=rust
 *
 * [494] 目标和
 */

// @lc code=start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if target.abs() > sum {
            return 0;
        }
        if (target + sum) % 2 == 1 {
            return 0;
        }
        let size = (target + sum) / 2;
        let mut dp = vec![0; size as usize + 1];
        dp[0] = 1;
        for i in nums {
            for s in (i as usize..=size as usize).rev() {
                dp[s] += dp[s - i as usize];
            }
        }
        dp[size as usize]
    }
}
// @lc code=end
