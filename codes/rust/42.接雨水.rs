/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut res = 0;
        while left < right {
            left_max = std::cmp::max(left_max, height[left]);
            right_max = std::cmp::max(right_max, height[right]);
            if height[left] < height[right] {
                res += left_max - height[left];
                left += 1;
            } else {
                res += right_max - height[right];
                right -= 1;
            }
        }
        res
    }
}
// @lc code=end
