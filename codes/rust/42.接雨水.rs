/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left_max_memo = vec![0; height.len()];
        let mut right_max_memo = vec![0; height.len()];
        for i in 1..height.len() - 1 {
            left_max_memo[i] = left_max_memo[i - 1].max(height[i - 1]);
        }
        for i in (1..height.len() - 1).rev() {
            right_max_memo[i] = right_max_memo[i + 1].max(height[i + 1]);
        }
        for i in 1..height.len() - 1 {
            let min = left_max_memo[i].min(right_max_memo[i]);
            if min > height[i] {
                res += min - height[i];
            }
        }
        res
    }
}
// @lc code=end
