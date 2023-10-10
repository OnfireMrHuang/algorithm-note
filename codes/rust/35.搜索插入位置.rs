/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut ans = nums.len() as i32;
        while left <= right {
            let mid = ((right - left) / 2) + left;
            if target > nums[mid as usize] {
                left = mid + 1;
            } else if target < nums[mid as usize] {
                ans = mid;
                right = mid - 1;
            } else {
                ans = mid;
                break;
            }
        }
        ans
    }
}
// @lc code=end
