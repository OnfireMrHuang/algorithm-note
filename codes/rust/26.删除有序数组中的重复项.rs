/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[fast] > nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }
            fast += 1;
        }
        (slow + 1) as i32
    }
}
// @lc code=end
