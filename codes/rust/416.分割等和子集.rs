/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 */

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 1 {
            return false;
        }
        let size = nums.len() as usize;
        let mut sum = 0;
        for num in &nums {
            sum += num;
        }
        if sum % 2 != 0 {
            return false;
        }
        sum = sum / 2;
        let mut dp = vec![vec![false; sum as usize + 1]; size + 1];
        // base case
        for i in 0..=size {
            dp[i][0] = true
        }
        // 动态转移方程
        for i in 1..=size {
            for j in 1..=sum as usize {
                if j < nums[i - 1] as usize {
                    dp[i as usize][j as usize] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize];
                }
            }
        }
        dp[size][sum as usize]
    }
}
// @lc code=end
