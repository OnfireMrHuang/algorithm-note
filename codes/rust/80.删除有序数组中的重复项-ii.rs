/*
 * @lc app=leetcode.cn id=80 lang=rust
 *
 * [80] 删除有序数组中的重复项 II
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::process(nums, 2)
    }

    fn process(nums: &mut Vec<i32>, k: i32) -> i32 {
        let mut idx: usize = 0;
        for i in 0..nums.len() {
            if idx < k as usize || nums[i] > nums[idx - k as usize] {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        idx as i32
    }
}
// @lc code=end
